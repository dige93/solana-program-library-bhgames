//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::GovernanceError,
    state::{enums::ProposalState, proposal::deserialize_proposal},
    tools::token::assert_spl_token_owner_is_signer,
};

/// Processes CancelProposal instruction
pub fn process_cancel_proposal(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let proposal_info = next_account_info(account_info_iter)?; // 0
    let admin_token_info = next_account_info(account_info_iter)?; // 1
    let proposal_owner_info = next_account_info(account_info_iter)?; // 2

    let mut proposal_data = deserialize_proposal(proposal_info)?;

    assert_spl_token_owner_is_signer(
        admin_token_info,
        &proposal_data.admin_mint,
        proposal_owner_info,
    )?;

    if !proposal_data.state.can_cancel() {
        return Err(GovernanceError::ProposalCannotBeCancelled.into());
    }

    proposal_data.state = ProposalState::Cancelled;
    proposal_data.serialize(&mut *proposal_info.data.borrow_mut())?;

    Ok(())
}
