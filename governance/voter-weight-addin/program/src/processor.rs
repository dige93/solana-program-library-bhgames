//! Program processor

use borsh::BorshDeserialize;
use spl_governance::addins::voter_weight::{VoterWeightAccountType, VoterWeightRecord};
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

    let governing_token_owner_info = next_account_info(account_info_iter)?; // 0
    let voter_weight_record_info = next_account_info(account_info_iter)?; // 1
    let payer_info = next_account_info(account_info_iter)?; // 2
    let system_info = next_account_info(account_info_iter)?; // 3

    let voter_weight_record_data = VoterWeightRecord {
        account_type: VoterWeightAccountType::VoterWeightRecord,
        governing_token_owner: *governing_token_owner_info.key,
        voter_weight: amount,
        voter_weight_at: Clock::get().unwrap().unix_timestamp,
    };

    create_and_serialize_account(
        payer_info,
        voter_weight_record_info,
        &voter_weight_record_data,
        program_id,
        system_info,
    )?;

    Ok(())
}
