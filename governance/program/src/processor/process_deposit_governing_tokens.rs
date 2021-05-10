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
        enums::GovernanceAccountType, governance_realm::GovernanceRealm, voter_record::VoterRecord,
    },
    tools::{
        account::{create_and_serialize_account, deserialize_account},
        token::transfer_spl_tokens,
    },
};

/// process_create_root_governance
pub fn process_deposit_governing_tokens(
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
    let payer_info = next_account_info(account_info_iter)?; // 4
    let system_info = next_account_info(account_info_iter)?; // 5
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

    transfer_spl_tokens(
        &governing_token_source_info,
        &governing_token_holding_info,
        &payer_info,
        amount,
        spl_token_info,
    )?;

    if voter_record_info.data_len() == 0 {
        let voter_record_data = VoterRecord {
            account_type: GovernanceAccountType::VoterRecord,
            governance_token_amount: governance_token_amount_delta,
            council_token_amount: council_token_amount_delta,
            active_votes_count: 0,
        };

        create_and_serialize_account(
            payer_info,
            voter_record_info,
            &voter_record_data,
            program_id,
            system_info,
        )?;
    } else {
        let mut voter_record_data =
            deserialize_account::<VoterRecord>(voter_record_info, program_id)?;

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
