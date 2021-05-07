//! Program state processor

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::proposal_state::{DESC_SIZE, NAME_SIZE};

/// process_create_proposal
pub fn process_create_proposal(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _description_link: &[u8; DESC_SIZE],
    _name: &[u8; NAME_SIZE],
) -> ProgramResult {
    Ok(())
}
