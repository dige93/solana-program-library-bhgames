//! Program state processor
use crate::utils::assert_program_upgrade_authority;
use crate::utils::create_account_raw;
use crate::{
    error::GovernanceError,
    state::enums::GovernanceAccountType,
    state::governance::{Governance, GOVERNANCE_NAME_LENGTH},
    PROGRAM_AUTHORITY_SEED,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
};

/// Init Governance
#[allow(clippy::too_many_arguments)]
pub fn process_create_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    vote_threshold: u8,

    minimum_slot_waiting_period: u64,
    time_limit: u64,
    name: [u8; GOVERNANCE_NAME_LENGTH],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let governance_account_info = next_account_info(account_info_iter)?; // 1
    let governed_program_account_info = next_account_info(account_info_iter)?; //2
    let governed_program_data_account_info = next_account_info(account_info_iter)?; // 3
    let governed_program_upgrade_authority_account_info = next_account_info(account_info_iter)?; // 4
    let governance_mint_account_info = next_account_info(account_info_iter)?; //5

    let payer_account_info = next_account_info(account_info_iter)?; // 6
    let system_account_info = next_account_info(account_info_iter)?; // 7
    let _bpf_upgrade_loader_account_info = next_account_info(account_info_iter)?; // 8

    let council_mint = next_account_info(account_info_iter) // 9?
        .map(|acc| Some(*acc.key))
        .unwrap_or(None);

    let mut seeds = vec![
        PROGRAM_AUTHORITY_SEED,
        governed_program_account_info.key.as_ref(),
    ];
    let (governance_key, bump_seed) = Pubkey::find_program_address(&seeds[..], program_id);
    if governance_account_info.key != &governance_key {
        return Err(GovernanceError::InvalidGovernanceKey.into());
    }

    // Assert current program upgrade authority signed the transaction as a temp. workaround until we can set_upgrade_authority via CPI.
    // Even though it doesn't transfer authority to the governance at the creation time it prevents from creating governance for programs owned by somebody else
    // After governance is created upgrade authority can be transferred to governance using CLI call.
    assert_program_upgrade_authority(
        &governance_key,
        governed_program_account_info.key,
        governed_program_data_account_info,
        governed_program_upgrade_authority_account_info,
    )?;

    // TODO: Uncomment once PR to allow set_upgrade_authority via CPI calls is released  https://github.com/solana-labs/solana/pull/16676
    // let set_upgrade_authority_ix = bpf_loader_upgradeable::set_upgrade_authority(
    //     &governed_program_account_info.key,
    //     &governed_program_upgrade_authority_account_info.key,
    //     Some(&governance_key),
    // );

    // let accounts = &[
    //     payer_account_info.clone(),
    //     bpf_upgrade_loader_account_info.clone(),
    //     governed_program_upgrade_authority_account_info.clone(),
    //     governance_account_info.clone(),
    //     governed_program_data_account_info.clone(),
    // ];
    // invoke(&set_upgrade_authority_ix, accounts)?;

    let bump = &[bump_seed];
    seeds.push(bump);

    create_account_raw::<Governance>(
        &[
            payer_account_info.clone(),
            governance_account_info.clone(),
            system_account_info.clone(),
        ],
        &governance_key,
        payer_account_info.key,
        program_id,
        &seeds[..],
    )?;

    let governance = Governance {
        account_type: GovernanceAccountType::Governance,
        name,
        minimum_slot_waiting_period,
        time_limit,
        program: *governed_program_account_info.key,
        governance_mint: *governance_mint_account_info.key,

        council_mint: council_mint,

        vote_threshold: vote_threshold,

        count: 0,
    };

    Governance::pack(governance, &mut governance_account_info.data.borrow_mut())?;

    Ok(())
}
