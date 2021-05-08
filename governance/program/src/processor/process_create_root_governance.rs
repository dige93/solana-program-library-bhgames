//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{enums::GovernanceAccountType, root_governance::RootGovernance},
    utils::create_and_serialize_account,
};

/// process_create_root_governance
pub fn process_create_root_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let root_governance_info = next_account_info(account_info_iter)?; // 1
    let payer_info = next_account_info(account_info_iter)?; // 3
    let system_info = next_account_info(account_info_iter)?; // 4

    let root_governance = RootGovernance {
        account_type: GovernanceAccountType::RootGovernance,
        name,
    };

    create_and_serialize_account::<RootGovernance>(
        payer_info.key,
        &root_governance_info,
        &root_governance,
        program_id,
        &[
            root_governance_info.clone(),
            payer_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
