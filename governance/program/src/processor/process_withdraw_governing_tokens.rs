//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::GovernanceError,
    state::{governance_realm::GovernanceRealm, voter_record::VoterRecord},
    tools::{
        accounts::deserialize_account, get_root_governance_address_seeds,
        token::transfer_spl_tokens_signed,
    },
};

/// process_withdraw_governing_tokens
pub fn process_withdraw_governing_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: Option<u64>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let root_governance_info = next_account_info(account_info_iter)?; // 1
    let governing_token_mint_info = next_account_info(account_info_iter)?; // 1
    let governing_token_holding_info = next_account_info(account_info_iter)?; // 1
    let governing_token_source_info = next_account_info(account_info_iter)?; // 1
    let voter_record_info = next_account_info(account_info_iter)?; // 1

    let spl_token_info = next_account_info(account_info_iter)?; // 6
    let _rent_sysvar_info = next_account_info(account_info_iter)?; // 7

    let root_governance_data =
        deserialize_account::<GovernanceRealm>(root_governance_info, program_id)?;

    let amount = amount.unwrap();

    let mut governance_token_amount_delta = 0;
    let mut council_token_amount_delta = 0;

    if root_governance_data.governance_mint == *governing_token_mint_info.key {
        governance_token_amount_delta = amount;
    } else if root_governance_data.council_mint == Some(*governing_token_mint_info.key) {
        council_token_amount_delta = amount;
    } else {
        return Err(GovernanceError::InvalidGoverningTokenMint.into());
    }

    transfer_spl_tokens_signed(
        &governing_token_holding_info,
        &governing_token_source_info,
        &root_governance_info,
        get_root_governance_address_seeds(&root_governance_data.name),
        program_id,
        amount,
        spl_token_info,
    )?;

    let mut voter_record_data = deserialize_account::<VoterRecord>(voter_record_info, program_id)?;

    voter_record_data.governance_token_amount = voter_record_data
        .governance_token_amount
        .checked_sub(governance_token_amount_delta)
        .unwrap();

    voter_record_data.council_token_amount = voter_record_data
        .council_token_amount
        .checked_sub(council_token_amount_delta)
        .unwrap();

    voter_record_data.serialize(&mut *voter_record_info.data.borrow_mut())?;

    Ok(())
}
