//! Program state processor

// use borsh::BorshSerialize;//
use solana_program::{
    //account_info::{next_account_info, AccountInfo},
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// use crate::{
//     error::GovernanceError,
//     state::{governance_realm::deserialize_realm, voter_record::deserialize_voter_record},
//     tools::{get_governance_realm_address_seeds, token::transfer_spl_tokens_signed},
// };

/// process_withdraw_governing_tokens
pub fn process_withdraw_governing_tokens(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _amount: Option<u64>,
) -> ProgramResult {
    // let account_info_iter = &mut accounts.iter();

    // let governance_realm_info = next_account_info(account_info_iter)?; // 1
    // let governing_token_mint_info = next_account_info(account_info_iter)?; // 2
    // let governing_token_holding_info = next_account_info(account_info_iter)?; // 3
    // let governing_token_source_info = next_account_info(account_info_iter)?; // 4
    // let voter_record_info = next_account_info(account_info_iter)?; // 5
    // let voter_info = next_account_info(account_info_iter)?; // 6
    // let spl_token_info = next_account_info(account_info_iter)?; // 7

    // if !voter_info.is_signer {
    //     return Err(GovernanceError::VoterMustSign.into());
    // }

    // let governance_realm_data = deserialize_realm(governance_realm_info)?;
    // let mut voter_record_data = deserialize_voter_record(voter_record_info, voter_info)?;

    // let mut governance_token_amount_delta = 0;
    // let mut council_token_amount_delta = 0;

    // let transfer_amount: u64;
    // let max_transfer_amount: u64;
    // let active_vote_count: u8;

    // if governance_realm_data.governance_mint == *governing_token_mint_info.key {
    //     active_vote_count = voter_record_data.active_governance_votes_count;
    //     governance_token_amount_delta =
    //         amount.unwrap_or(voter_record_data.governance_token_deposit_amount);
    //     transfer_amount = governance_token_amount_delta;
    //     max_transfer_amount = voter_record_data.governance_token_deposit_amount;
    // } else if governance_realm_data.council_mint == Some(*governing_token_mint_info.key) {
    //     active_vote_count = voter_record_data.active_council_votes_count;
    //     council_token_amount_delta =
    //         amount.unwrap_or(voter_record_data.council_token_deposit_amount);
    //     transfer_amount = council_token_amount_delta;
    //     max_transfer_amount = voter_record_data.council_token_deposit_amount;
    // } else {
    //     return Err(GovernanceError::InvalidGoverningTokenMint.into());
    // }

    // if active_vote_count > 0 {
    //     return Err(GovernanceError::CannotWithdrawGoverningTokensWhenActiveVotesExist.into());
    // }

    // if transfer_amount > max_transfer_amount {
    //     return Err(GovernanceError::CannotWithdrawMoreGoverningTokensThenDeposited.into());
    // }

    // transfer_spl_tokens_signed(
    //     &governing_token_holding_info,
    //     &governing_token_source_info,
    //     &governance_realm_info,
    //     get_governance_realm_address_seeds(&governance_realm_data.name),
    //     program_id,
    //     transfer_amount,
    //     spl_token_info,
    // )?;

    // voter_record_data.governance_token_deposit_amount = voter_record_data
    //     .governance_token_deposit_amount
    //     .checked_sub(governance_token_amount_delta)
    //     .unwrap();

    // voter_record_data.council_token_deposit_amount = voter_record_data
    //     .council_token_deposit_amount
    //     .checked_sub(council_token_amount_delta)
    //     .unwrap();

    // voter_record_data.serialize(&mut *voter_record_info.data.borrow_mut())?;

    Ok(())
}
