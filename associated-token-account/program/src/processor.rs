//! Program state processor

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

    let funder_info = next_account_info(account_info_iter)?;
    let associated_token_account_info = next_account_info(account_info_iter)?;
    let wallet_account_info = next_account_info(account_info_iter)?;
    let spl_token_mint_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;
    let spl_token_program_id = spl_token_program_info.key;
    let rent_sysvar_info = next_account_info(account_info_iter)?;

    let (associated_token_address, bump_seed) = get_associated_token_address_and_bump_seed_internal(
        wallet_account_info.key,
        spl_token_mint_info.key,
        program_id,
        spl_token_program_id,
    );
    if associated_token_address != *associated_token_account_info.key {
        msg!("Error: Associated address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }

    let associated_token_account_signer_seeds: &[&[_]] = &[
        &wallet_account_info.key.to_bytes(),
        &spl_token_program_id.to_bytes(),
        &spl_token_mint_info.key.to_bytes(),
        &[bump_seed],
    ];

    let rent = &Rent::from_account_info(rent_sysvar_info)?;

    create_pda_account(
        funder_info,
        rent,
        spl_token::state::Account::LEN,
        &spl_token::id(),
        system_program_info,
        associated_token_account_info,
        associated_token_account_signer_seeds,
    )?;

    msg!("Initialize the associated token account");
    invoke(
        &spl_token::instruction::initialize_account(
            spl_token_program_id,
            associated_token_account_info.key,
            spl_token_mint_info.key,
            wallet_account_info.key,
        )?,
        &[
            associated_token_account_info.clone(),
            spl_token_mint_info.clone(),
            wallet_account_info.clone(),
            rent_sysvar_info.clone(),
            spl_token_program_info.clone(),
        ],
    )
}

/// Processes CreateAssociatedTokenAccount instruction
pub fn process_mint_to(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let wallet_info = next_account_info(account_info_iter)?;
    let associated_account_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let rent = Rent::get()?;

    Ok(())
}
