//! Program state processor
use crate::{
    error::GovernanceError,
    state::account_governance::AccountGovernance,
    state::{
        enums::ProposalStateStatus,
        z_custom_single_signer_transaction::{CustomSingleSignerTransaction, MAX_ACCOUNTS_ALLOWED},
        z_proposal::ProposalOld,
        z_proposal_state::ProposalState,
    },
    utils::{
        assert_account_equiv, assert_executing, assert_initialized, assert_initialized_old,
        execute, ExecuteParams,
    },
    PROGRAM_AUTHORITY_SEED,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    instruction::Instruction,
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

/// Execute an instruction
pub fn process_execute(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let transaction_account_info = next_account_info(account_info_iter)?;
    let proposal_state_account_info = next_account_info(account_info_iter)?;
    let program_to_invoke_info = next_account_info(account_info_iter)?;
    let proposal_account_info = next_account_info(account_info_iter)?;
    let governance_account_info = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;

    let mut proposal_state: ProposalState = assert_initialized(proposal_state_account_info)?;
    let proposal: ProposalOld = assert_initialized(proposal_account_info)?;
    let governance: AccountGovernance = assert_initialized_old(governance_account_info)?;
    let clock = &Clock::from_account_info(clock_info)?;
    // For now we assume all transactions are CustomSingleSignerTransactions even though
    // this will not always be the case...we need to solve that inheritance issue later.
    let mut transaction: CustomSingleSignerTransaction =
        assert_initialized(transaction_account_info)?;

    let time_elapsed = match clock.slot.checked_sub(proposal_state.voting_ended_at) {
        Some(val) => val,
        None => return Err(GovernanceError::NumericalOverflow.into()),
    };

    if time_elapsed < transaction.delay_slots {
        return Err(GovernanceError::TooEarlyToExecute.into());
    }

    assert_account_equiv(proposal_state_account_info, &proposal.state)?;
    assert_account_equiv(governance_account_info, &proposal.governance)?;

    let mut seeds = vec![PROGRAM_AUTHORITY_SEED, governance.governed_account.as_ref()];

    let (governance_authority, bump_seed) = Pubkey::find_program_address(&seeds[..], program_id);

    let mut account_infos: Vec<AccountInfo> = vec![];

    let mut added_authority = false;

    loop {
        if let Ok(next_account) = next_account_info(account_info_iter) {
            // TODO: Review this check. Can't we just check for the governance key and allow other governance accounts?
            if next_account.data_len() == 1000 {
                // check it's governance account using sie
                // You better be initialized, and if you are, you better at least be mine...
                let _nefarious_governance: AccountGovernance =
                    assert_initialized_old(&next_account)?;
                assert_account_equiv(&next_account, &proposal.governance)?;
                added_authority = true;
                if next_account.key != &governance_authority {
                    return Err(GovernanceError::InvalidGovernanceKey.into());
                }
            }
            account_infos.push(next_account.clone());
        } else {
            break;
        }
    }

    if account_infos.len() > (MAX_ACCOUNTS_ALLOWED - 2) {
        return Err(GovernanceError::TooManyAccountsInInstruction.into());
    }

    account_infos.push(program_to_invoke_info.clone());

    if !added_authority {
        if governance_account_info.key != &governance_authority {
            return Err(GovernanceError::InvalidGovernanceKey.into());
        }
        account_infos.push(governance_account_info.clone());
    }

    assert_executing(&proposal_state)?;

    if transaction.executed == 1 {
        return Err(GovernanceError::ProposalTransactionAlreadyExecuted.into());
    }

    let message: Message = match bincode::deserialize::<Message>(
        &transaction.instruction[0..transaction.instruction_end_index as usize + 1],
    ) {
        Ok(val) => val,
        Err(_) => return Err(GovernanceError::InstructionUnpackError.into()),
    };

    // TODO: Once demote_sysvar_write_locks features is enabled we should pass true
    let serialized_instructions = message.serialize_instructions(false);
    let instruction: Instruction =
        match Message::deserialize_instruction(0, &serialized_instructions) {
            Ok(val) => val,
            Err(_) => return Err(GovernanceError::InstructionUnpackError.into()),
        };

    let bump = &[bump_seed];
    seeds.push(bump);
    let authority_signer_seeds = &seeds[..];

    execute(ExecuteParams {
        instruction,
        authority_signer_seeds,
        account_infos,
    })?;

    transaction.executed = 1;

    CustomSingleSignerTransaction::pack(
        transaction,
        &mut transaction_account_info.data.borrow_mut(),
    )?;

    proposal_state.number_of_executed_transactions = match proposal_state
        .number_of_executed_transactions
        .checked_add(1)
    {
        Some(val) => val,
        None => return Err(GovernanceError::NumericalOverflow.into()),
    };

    if proposal_state.number_of_executed_transactions == proposal_state.number_of_transactions {
        proposal_state.status = ProposalStateStatus::Completed
    }

    ProposalState::pack(
        proposal_state,
        &mut proposal_state_account_info.data.borrow_mut(),
    )?;
    Ok(())
}
