//! Program processor

use crate::instruction::GovernanceChatInstruction;
use borsh::BorshDeserialize;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GovernanceChatInstruction::try_from_slice(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("GOVERNANCE-CHAT-INSTRUCTION: {:?}", instruction);

    match instruction {
        GovernanceChatInstruction::PostMessage {} => process_post_message(program_id, accounts),
    }
}

/// Processes PostMessage instruction
pub fn process_post_message(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}
