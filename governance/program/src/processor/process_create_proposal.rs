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
        account_governance::{deserialize_account_governance, AccountGovernance},
        enums::{GovernanceAccountType, GoverningTokenType, ProposalState},
        proposal::{get_proposal_address_seeds, Proposal},
    },
    tools::{
        account::create_and_serialize_account_signed,
        token::{create_spl_token_account, create_spl_token_mint},
    },
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

    let admin_mint_info = next_account_info(account_info_iter)?; // 1
    let admin_token_info = next_account_info(account_info_iter)?; // 1

    let signatory_mint_info = next_account_info(account_info_iter)?; // 1
    let signatory_token_info = next_account_info(account_info_iter)?; // 1

    let proposal_owner_info = next_account_info(account_info_iter)?; // 2

    let payer_info = next_account_info(account_info_iter)?; // 2
    let system_info = next_account_info(account_info_iter)?; // 3
    let spl_token_info = next_account_info(account_info_iter)?; // 7
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 7

    if !proposal_info.data_is_empty() {
        return Err(GovernanceError::ProposalAlreadyExists.into());
    }

    create_spl_token_mint(
        payer_info,
        admin_mint_info,
        proposal_info.key,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    create_spl_token_account(
        payer_info,
        admin_token_info,
        admin_mint_info,
        proposal_owner_info,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    create_spl_token_mint(
        payer_info,
        signatory_mint_info,
        proposal_info.key,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    create_spl_token_account(
        payer_info,
        signatory_token_info,
        signatory_mint_info,
        proposal_owner_info,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    let proposal_data = Proposal {
        account_type: GovernanceAccountType::Proposal,
        name: name.clone(),
        description_link,
        account_governance: *account_governance_info.key,
        governing_token_type,
        state: ProposalState::Draft,
    };

    create_and_serialize_account_signed::<Proposal>(
        payer_info,
        proposal_info,
        &proposal_data,
        get_proposal_address_seeds(account_governance_info.key, &name),
        program_id,
        system_info,
    )?;

    let mut account_governance_data: AccountGovernance =
        deserialize_account_governance(account_governance_info)?;

    account_governance_data.proposal_count = account_governance_data
        .proposal_count
        .checked_add(1)
        .unwrap();

    account_governance_data.serialize(&mut *account_governance_info.data.borrow_mut())?;

    Ok(())
}
