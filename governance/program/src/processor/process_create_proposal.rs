//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{enums::GovernanceAccountType, proposal::Proposal},
    utils::create_serialized_account,
};

/// process_create_proposal
pub fn process_create_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    description_link: String,
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let proposal_info = next_account_info(account_info_iter)?; // 1
    let payer_info = next_account_info(account_info_iter)?; // 2
    let system_info = next_account_info(account_info_iter)?; // 3

    let proposal = Proposal {
        account_type: GovernanceAccountType::Proposal,
        name,
        description_link,
    };

    create_serialized_account::<Proposal>(
        payer_info.key,
        &proposal_info,
        &proposal,
        program_id,
        &[
            proposal_info.clone(),
            payer_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
