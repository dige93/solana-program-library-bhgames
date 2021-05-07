//! Program state processor

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{enums::GovernanceAccountType, proposal::Proposal},
    utils::create_account_raw2,
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
        name: name,
        description_link: description_link,
    };

    let data = proposal.try_to_vec()?;

    create_account_raw2::<Proposal>(
        &[
            payer_info.clone(),
            proposal_info.clone(),
            system_info.clone(),
        ],
        &proposal_info.key,
        payer_info.key,
        program_id,
        data.len(),
    )?;

    proposal_info.data.borrow_mut().copy_from_slice(&data);

    Ok(())
}
