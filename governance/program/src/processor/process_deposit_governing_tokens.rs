//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::GovernanceError,
    state::{
        enums::GovernanceAccountType,
        governance_realm::deserialize_governance_realm,
        voter_record::{deserialize_voter_record, VoterRecord},
    },
    tools::{
        account::create_and_serialize_account,
        token::{get_amount_from_token_account, transfer_spl_tokens},
    },
};

/// process deposit governing tokens
pub fn process_deposit_governing_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: Option<u64>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let governance_realm_info = next_account_info(account_info_iter)?; // 1
    let governing_token_mint_info = next_account_info(account_info_iter)?; // 2
    let governing_token_holding_info = next_account_info(account_info_iter)?; // 3
    let governing_token_source_info = next_account_info(account_info_iter)?; // 4
    let voter_record_info = next_account_info(account_info_iter)?; // 5
    let voter_info = next_account_info(account_info_iter)?; // 6
    let system_info = next_account_info(account_info_iter)?; // 7
    let spl_token_info = next_account_info(account_info_iter)?; // 8

    let governance_realm_data = deserialize_governance_realm(governance_realm_info)?;

    let amount = amount
        .unwrap_or_else(|| get_amount_from_token_account(governing_token_source_info).unwrap());

    let mut governance_token_amount_delta = 0;
    let mut council_token_amount_delta = 0;

    if governance_realm_data.governance_mint == *governing_token_mint_info.key {
        governance_token_amount_delta = amount;
    } else if governance_realm_data.council_mint == Some(*governing_token_mint_info.key) {
        council_token_amount_delta = amount;
    } else {
        return Err(GovernanceError::InvalidGoverningTokenMint.into());
    }

    transfer_spl_tokens(
        &governing_token_source_info,
        &governing_token_holding_info,
        &voter_info,
        amount,
        spl_token_info,
    )?;

    if voter_record_info.data_len() == 0 {
        let voter_record_data = VoterRecord {
            account_type: GovernanceAccountType::VoterRecord,
            voter: *voter_info.key,
            governance_token_amount: governance_token_amount_delta,
            council_token_amount: council_token_amount_delta,
            active_votes_count: 0,
        };

        create_and_serialize_account(
            voter_info,
            voter_record_info,
            &voter_record_data,
            program_id,
            system_info,
        )?;
    } else {
        let mut voter_record_data = deserialize_voter_record(voter_record_info, voter_info)?;

        voter_record_data.governance_token_amount = voter_record_data
            .governance_token_amount
            .checked_add(governance_token_amount_delta)
            .unwrap();

        voter_record_data.council_token_amount = voter_record_data
            .council_token_amount
            .checked_add(council_token_amount_delta)
            .unwrap();

        voter_record_data.serialize(&mut *voter_record_info.data.borrow_mut())?;
    }

    Ok(())
}
