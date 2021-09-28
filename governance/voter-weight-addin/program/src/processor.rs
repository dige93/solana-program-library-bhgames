//! Program processor

use borsh::BorshDeserialize;
use spl_governance::{
    addins::voter_weight::{VoterWeightAccountType, VoterWeightRecord},
    state::{
        realm::get_realm_data,
        token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint,
    },
};
// TODO: Move to shared governance tools
use spl_governance_chat::tools::account::create_and_serialize_account;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::instruction::VoterWeightAddinInstruction;

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = VoterWeightAddinInstruction::try_from_slice(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("GOVERNANCE-VOTER-WEIGHT-INSTRUCTION: {:?}", instruction);

    match instruction {
        VoterWeightAddinInstruction::Revise { time_offset: _ } => Ok(()),
        VoterWeightAddinInstruction::Deposit { amount } => {
            process_deposit(program_id, accounts, amount)
        }
    }
}

/// Processes Deposit instruction
pub fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let governance_program_info = next_account_info(account_info_iter)?; // 0
    let realm_info = next_account_info(account_info_iter)?; // 1
    let governing_token_owner_info = next_account_info(account_info_iter)?; // 2
    let voter_weight_info = next_account_info(account_info_iter)?; // 3
    let payer_info = next_account_info(account_info_iter)?; // 4
    let system_info = next_account_info(account_info_iter)?; // 5

    let realm_data = get_realm_data(governance_program_info.key, realm_info)?;
    let governing_token_owner_data = get_token_owner_record_data_for_realm_and_governing_mint(
        governance_program_info.key,
        governing_token_owner_info,
        realm_info.key,
        &realm_data.community_mint,
    )?;

    // TODO: Custom deposit logic goes here
    // Note: Assert realm community mint and the deposit mint match

    let voter_weight_data = VoterWeightRecord {
        account_type: VoterWeightAccountType::VoterWeightRecord,
        governing_token_owner: governing_token_owner_data.governing_token_owner,
        voter_weight: amount,
        voter_weight_at: Clock::get().unwrap().unix_timestamp,
        voter_weight_expiry: None,
        realm: *realm_info.key,
    };

    create_and_serialize_account(
        payer_info,
        voter_weight_info,
        &voter_weight_data,
        program_id,
        system_info,
    )?;

    Ok(())
}
