//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::{
    state::{
        enums::GovernanceAccountType,
        proposal::Proposal,
        proposal_state::{DESC_SIZE, NAME_SIZE},
    },
    utils::{create_account_raw, create_account_raw2},
};

/// process_create_proposal
pub fn process_create_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    description_link: &[u8; DESC_SIZE],
    name: &[u8; NAME_SIZE],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let proposal_info = next_account_info(account_info_iter)?; // 1

    let payer_info = next_account_info(account_info_iter)?; // 2
    let system_info = next_account_info(account_info_iter)?; // 3

    create_account_raw2::<Proposal>(
        &[
            payer_info.clone(),
            proposal_info.clone(),
            system_info.clone(),
        ],
        &proposal_info.key,
        payer_info.key,
        program_id,
    )?;

    let proposal = Proposal {
        account_type: GovernanceAccountType::Proposal,
        description_link: *description_link,
        name: *name,
    };

    Proposal::pack(proposal, &mut proposal_info.data.borrow_mut())?;

    Ok(())
}
