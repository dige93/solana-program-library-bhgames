//! Program state processor
use crate::utils::create_account_raw;
use crate::{
    error::GovernanceError,
    state::enums::{ExecutionType, GovernanceAccountType, GovernanceType, VotingEntryRule},
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
pub fn process_init_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    vote_threshold: u8,
    execution_type: u8,
    governance_type: u8,
    voting_entry_rule: u8,
    minimum_slot_waiting_period: u64,
    time_limit: u64,
    name: [u8; GOVERNANCE_NAME_LENGTH],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let governance_account_info = next_account_info(account_info_iter)?;
    let governed_program_account_info = next_account_info(account_info_iter)?;
    let governance_mint_account_info = next_account_info(account_info_iter)?;

    let payer_account_info = next_account_info(account_info_iter)?; // 4
    let system_account_info = next_account_info(account_info_iter)?; // 5

    let (council_mint, _) = next_account_info(account_info_iter)
        .map(|acc| (Some(*acc.key), acc.key.as_ref()))
        .unwrap_or((None, &[]));

    let mut seeds = vec![
        PROGRAM_AUTHORITY_SEED,
        governed_program_account_info.key.as_ref(),
    ];
    let (governance_key, bump_seed) = Pubkey::find_program_address(&seeds[..], program_id);
    if governance_account_info.key != &governance_key {
        return Err(GovernanceError::InvalidGovernanceKey.into());
    }

    let bump = &[bump_seed];
    seeds.push(bump);
    let authority_signer_seeds = &seeds[..];

    create_account_raw::<Governance>(
        &[
            payer_account_info.clone(),
            governance_account_info.clone(),
            system_account_info.clone(),
        ],
        &governance_key,
        payer_account_info.key,
        program_id,
        authority_signer_seeds,
    )?;

    let new_governance = Governance {
        account_type: GovernanceAccountType::Governance,
        name,
        minimum_slot_waiting_period,
        time_limit,
        program: *governed_program_account_info.key,
        governance_mint: *governance_mint_account_info.key,

        council_mint: council_mint,

        vote_threshold: vote_threshold,
        execution_type: match execution_type {
            0 => ExecutionType::Independent,
            _ => ExecutionType::Independent,
        },

        governance_type: match governance_type {
            0 => GovernanceType::Governance,
            _ => GovernanceType::Governance,
        },

        voting_entry_rule: match voting_entry_rule {
            0 => VotingEntryRule::Anytime,
            _ => VotingEntryRule::Anytime,
        },
        count: 0,
    };

    Governance::pack(
        new_governance,
        &mut governance_account_info.data.borrow_mut(),
    )?;

    Ok(())
}
