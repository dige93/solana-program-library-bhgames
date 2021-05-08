//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{
        enums::GovernanceAccountType, root_governance::RootGovernance, voter_record::VoterRecord,
    },
    tools::accounts::{create_and_serialize_account, deserialize_account},
};

/// process_create_root_governance
pub fn process_deposit_governing_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: Option<u64>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let root_governance_info = next_account_info(account_info_iter)?; // 1
    let _governing_token_mint_info = next_account_info(account_info_iter)?; // 1
    let _governing_token_holding_info = next_account_info(account_info_iter)?; // 1
    let _governing_token_source_info = next_account_info(account_info_iter)?; // 1
    let voter_record_info = next_account_info(account_info_iter)?; // 1
    let payer_info = next_account_info(account_info_iter)?; // 4
    let system_info = next_account_info(account_info_iter)?; // 5
    let _spl_token_info = next_account_info(account_info_iter)?; // 6
    let _rent_sysvar_info = next_account_info(account_info_iter)?; // 7

    let _root_governance_data =
        deserialize_account::<RootGovernance>(root_governance_info, program_id);

    let voter_record_data = VoterRecord {
        account_type: GovernanceAccountType::VoteRecord,
        governance_token_amount: amount.unwrap(),
        council_token_amount: None,
        active_votes_count: 0,
    };

    create_and_serialize_account(
        &payer_info.key,
        voter_record_info,
        &voter_record_data,
        program_id,
        &[
            voter_record_info.clone(),
            payer_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
