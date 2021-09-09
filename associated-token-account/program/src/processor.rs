//! Program state processor

use crate::tools::mint_spl_tokens_to;
use crate::*;
use crate::{instruction::AssociatedTokenAccountInstruction, tools::create_pda_account};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = if input.is_empty() {
        AssociatedTokenAccountInstruction::CreateAssociatedTokenAccount
    } else {
        AssociatedTokenAccountInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?
    };

    msg!("ASSOCIATED-TOKEN-ACCOUNT-INSTRUCTION: {:?}", instruction);

    match instruction {
        AssociatedTokenAccountInstruction::CreateAssociatedTokenAccount {} => {
            process_create_associated_token_account(program_id, accounts)
        }
        AssociatedTokenAccountInstruction::MintTo { amount } => {
            process_mint_to(program_id, accounts, amount)
        }
    }
}

/// Processes CreateAssociatedTokenAccount instruction
pub fn process_create_associated_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let payer_info = next_account_info(account_info_iter)?;
    let associated_account_info = next_account_info(account_info_iter)?;
    let wallet_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;

    create_associated_token_account(
        program_id,
        mint_info,
        wallet_info,
        associated_account_info,
        payer_info,
        spl_token_info,
        system_info,
        rent_sysvar_info,
    )
}

/// Processes MintTo instruction
pub fn process_mint_to(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_info = next_account_info(account_info_iter)?; // 0
    let mint_authority_info = next_account_info(account_info_iter)?; // 1
    let wallet_info = next_account_info(account_info_iter)?; // 2
    let associated_account_info = next_account_info(account_info_iter)?; // 3
    let payer_info = next_account_info(account_info_iter)?; // 4
    let spl_token_info = next_account_info(account_info_iter)?; // 5
    let system_info = next_account_info(account_info_iter)?; // 6
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 7

    // TODO: Check if wallet is spl token account for the same mint
    // Reject none system accounts?  Should we mint to SPL token account anyway?
    // Check if ATA exists before creating it

    create_associated_token_account(
        program_id,
        mint_info,
        wallet_info,
        associated_account_info,
        payer_info,
        spl_token_info,
        system_info,
        rent_sysvar_info,
    )?;

    mint_spl_tokens_to(
        mint_info,
        associated_account_info,
        mint_authority_info,
        amount,
        spl_token_info,
    )
}

/// Processes CreateAssociatedTokenAccount instruction
fn create_associated_token_account<'a>(
    program_id: &Pubkey,
    mint_info: &AccountInfo<'a>,
    wallet_info: &AccountInfo<'a>,
    associated_account_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
) -> ProgramResult {
    let (associated_token_address, bump_seed) = get_associated_token_address_and_bump_seed_internal(
        wallet_info.key,
        mint_info.key,
        program_id,
        spl_token_info.key,
    );
    if associated_token_address != *associated_account_info.key {
        msg!("Error: Associated address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }

    let associated_token_account_signer_seeds: &[&[_]] = &[
        &wallet_info.key.to_bytes(),
        &spl_token_info.key.to_bytes(),
        &mint_info.key.to_bytes(),
        &[bump_seed],
    ];

    let rent = &Rent::from_account_info(rent_sysvar_info)?;

    create_pda_account(
        payer_info,
        rent,
        spl_token::state::Account::LEN,
        &spl_token::id(),
        system_info,
        associated_account_info,
        associated_token_account_signer_seeds,
    )?;

    msg!("Initialize the associated token account");
    invoke(
        &spl_token::instruction::initialize_account(
            spl_token_info.key,
            associated_account_info.key,
            mint_info.key,
            wallet_info.key,
        )?,
        &[
            associated_account_info.clone(),
            mint_info.clone(),
            wallet_info.clone(),
            rent_sysvar_info.clone(),
            spl_token_info.clone(),
        ],
    )
}
