//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{
        account_governance::{deserialize_account_governance, AccountGovernance},
        enums::{GovernanceAccountType, GoverningTokenType, ProposalState},
        proposal::Proposal,
    },
    tools::account::create_and_serialize_account,
};

/// process_create_proposal
pub fn process_create_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    governing_token_type: GoverningTokenType,
    description_link: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let proposal_info = next_account_info(account_info_iter)?; // 0
    let account_governance_info = next_account_info(account_info_iter)?; // 1
    let payer_info = next_account_info(account_info_iter)?; // 2
    let system_info = next_account_info(account_info_iter)?; // 3

    let mut account_governance_data: AccountGovernance =
        deserialize_account_governance(account_governance_info)?;

    let proposal_data = Proposal {
        account_type: GovernanceAccountType::Proposal,
        name,
        description_link,
        account_governance: *account_governance_info.key,
        governing_token_type,
        state: ProposalState::Draft,
    };

    create_and_serialize_account::<Proposal>(
        payer_info,
        proposal_info,
        &proposal_data,
        program_id,
        system_info,
    )?;

    account_governance_data.proposal_count = account_governance_data
        .proposal_count
        .checked_add(1)
        .unwrap();

    account_governance_data.serialize(&mut *account_governance_info.data.borrow_mut())?;

    Ok(())
}
