//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{enums::GovernanceAccountType, root_governance::RootGovernance},
    tools::get_root_governance_address_with_bump_seed,
    utils::{create_and_serialize_account, create_and_serialize_account_signed},
    PROGRAM_AUTHORITY_SEED,
};

/// process_create_root_governance
pub fn process_create_root_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let root_governance_info = next_account_info(account_info_iter)?; // 1
    let governance_mint_info = next_account_info(account_info_iter)?; // 2
    let payer_info = next_account_info(account_info_iter)?; // 3
    let system_info = next_account_info(account_info_iter)?; // 4

    let council_mint_key = next_account_info(account_info_iter) // 5
        .map(|ai| Some(*ai.key))
        .unwrap_or(None);

    let (root_governance_address, bump_seed) =
        get_root_governance_address_with_bump_seed(&name, Some(root_governance_info.key))?;

    let name_seed = name.clone();
    let mut seeds = vec![PROGRAM_AUTHORITY_SEED, &name_seed.as_bytes()];

    let root_governance = RootGovernance {
        account_type: GovernanceAccountType::RootGovernance,
        governance_mint: *governance_mint_info.key,
        council_mint: council_mint_key,
        name,
    };

    let bump = &[bump_seed];
    seeds.push(bump);

    create_and_serialize_account_signed::<RootGovernance>(
        payer_info.key,
        &root_governance_info,
        &root_governance,
        program_id,
        &[
            root_governance_info.clone(),
            payer_info.clone(),
            system_info.clone(),
        ],
        &seeds[..],
    )?;

    Ok(())
}
