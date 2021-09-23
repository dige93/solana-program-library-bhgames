//! Program processor

use borsh::BorshDeserialize;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::GovernanceStakePoolInstruction;

/// Processes an instruction
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GovernanceStakePoolInstruction::try_from_slice(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("GOVERNANCE-STAKE-POOL-INSTRUCTION: {:?}", instruction);

    match instruction {
        GovernanceStakePoolInstruction::Deposit {} => Ok(()),
    }
}
