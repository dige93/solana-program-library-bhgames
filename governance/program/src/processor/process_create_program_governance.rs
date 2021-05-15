//! Program state processor
//use crate::utils::assert_program_upgrade_authority;

use crate::{
    state::program_governance::ProgramGovernance,
    state::{
        enums::GovernanceAccountType, program_governance::get_program_governance_address_seeds,
    },
    tools::{
        account::create_and_serialize_account_signed, bpf_loader::assert_program_upgrade_authority,
    },
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

/// Init Governance
#[allow(clippy::too_many_arguments)]
pub fn process_create_program_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    vote_threshold: u8,

    minimum_slot_waiting_period: u64,
    time_limit: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let program_governance_info = next_account_info(account_info_iter)?; // 0
    let governed_program_info = next_account_info(account_info_iter)?; // 1
    let governed_program_data_info = next_account_info(account_info_iter)?; // 2
    let governed_program_upgrade_authority_info = next_account_info(account_info_iter)?; // 3
    let governance_mint_info = next_account_info(account_info_iter)?; // 4

    let payer_info = next_account_info(account_info_iter)?; // 5
    let system_info = next_account_info(account_info_iter)?; // 6
    let _bpf_upgrade_loader_account_info = next_account_info(account_info_iter)?; // 7

    let council_mint_key = next_account_info(account_info_iter) // 8?
        .map(|acc| Some(*acc.key))
        .unwrap_or(None);

    // Assert current program upgrade authority signed the transaction as a temp. workaround until we can set_upgrade_authority via CPI.
    // Even though it doesn't transfer authority to the governance at the creation time it prevents from creating governance for programs owned by somebody else
    // After governance is created upgrade authority can be transferred to governance using CLI call.

    assert_program_upgrade_authority(
        &program_governance_info.key,
        governed_program_info.key,
        governed_program_data_info,
        governed_program_upgrade_authority_info,
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

    let program_governance_data = ProgramGovernance {
        account_type: GovernanceAccountType::ProgramGovernance,

        min_instruction_hold_up_time: minimum_slot_waiting_period,
        max_voting_time: time_limit,
        program: *governed_program_info.key,
        governance_mint: *governance_mint_info.key,

        council_mint: council_mint_key,

        vote_threshold: vote_threshold,

        proposal_count: 0,
    };

    create_and_serialize_account_signed::<ProgramGovernance>(
        payer_info,
        &program_governance_info,
        &program_governance_data,
        get_program_governance_address_seeds(governed_program_info.key),
        program_id,
        system_info,
    )?;

    Ok(())
}
