pub mod process_add_custom_single_signer_transaction;
pub mod process_add_signer;
pub mod process_create_account_governance;
pub mod process_create_empty_governance_voting_record;
pub mod process_create_program_governance;

pub mod process_create_proposal;
pub mod process_create_realm;
pub mod process_delete_proposal;
pub mod process_deposit_governing_tokens;
pub mod process_deposit_source_tokens;
pub mod process_execute;
pub mod process_init_proposal;
pub mod process_remove_signer;
pub mod process_remove_transaction;
pub mod process_set_vote_authority;
pub mod process_sign;
pub mod process_update_transaction_slot;
pub mod process_vote;
pub mod process_withdraw_governing_tokens;
pub mod process_withdraw_voting_tokens;

use crate::instruction::GovernanceInstruction;
use borsh::BorshDeserialize;
use process_add_custom_single_signer_transaction::process_add_custom_single_signer_transaction;
use process_add_signer::process_add_signer;
use process_create_account_governance::process_create_account_governance;
use process_create_empty_governance_voting_record::process_create_empty_governance_voting_record;
use process_create_program_governance::process_create_program_governance;
use process_create_proposal::process_create_proposal;
use process_create_realm::process_create_realm;
use process_delete_proposal::process_cancel_proposal;
use process_deposit_governing_tokens::process_deposit_governing_tokens;
use process_deposit_source_tokens::process_deposit_source_tokens;
use process_execute::process_execute;
use process_init_proposal::process_init_proposal;
use process_remove_signer::process_remove_signer;
use process_remove_transaction::process_remove_transaction;
use process_set_vote_authority::process_set_vote_authority;
use process_sign::process_sign;
use process_update_transaction_slot::process_update_transaction_slot;
use process_vote::process_vote;
use process_withdraw_governing_tokens::process_withdraw_governing_tokens;
use process_withdraw_voting_tokens::process_withdraw_voting_tokens;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GovernanceInstruction::try_from_slice(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        GovernanceInstruction::InitProposal {
            name,
            description_link: desc_link,
        } => {
            msg!("Instruction: Init Proposal");
            process_init_proposal(program_id, accounts, &name, &desc_link)
        }
        GovernanceInstruction::AddSignatory => {
            msg!("Instruction: Add Signer");
            process_add_signer(program_id, accounts)
        }
        GovernanceInstruction::RemoveSignatory => {
            msg!("Instruction: Remove Signer");
            process_remove_signer(program_id, accounts)
        }
        GovernanceInstruction::AddCustomSingleSignerTransaction {
            delay_slots,
            instruction,
            position,
            instruction_end_index,
        } => process_add_custom_single_signer_transaction(
            program_id,
            accounts,
            delay_slots,
            instruction,
            position,
            instruction_end_index,
        ),
        GovernanceInstruction::RemoveTransaction => {
            msg!("Instruction: Remove Transaction");
            process_remove_transaction(program_id, accounts)
        }
        GovernanceInstruction::UpdateTransactionDelaySlots { delay_slots } => {
            msg!("Instruction: Update Transaction Slot");
            process_update_transaction_slot(program_id, accounts, delay_slots)
        }
        GovernanceInstruction::CancelProposal => {
            msg!("Instruction: Delete Proposal");
            process_cancel_proposal(program_id, accounts)
        }
        GovernanceInstruction::SignProposal => {
            msg!("Instruction: Sign");
            process_sign(program_id, accounts)
        }
        GovernanceInstruction::Vote { vote } => {
            msg!("Instruction: Vote");
            process_vote(program_id, accounts, vote)
        }
        GovernanceInstruction::CreateProgramGovernance {
            realm,
            governed_program,
            vote_threshold,
            min_instruction_hold_up_time,
            max_voting_time,
            token_threshold_to_create_proposal,
        } => {
            msg!("Instruction: Initialize Governance");
            process_create_program_governance(
                program_id,
                accounts,
                &realm,
                &governed_program,
                vote_threshold,
                min_instruction_hold_up_time,
                max_voting_time,
                token_threshold_to_create_proposal,
            )
        }
        GovernanceInstruction::CreateAccountGovernance {
            realm,
            governed_account,
            vote_threshold,
            min_instruction_hold_up_time,
            max_voting_time,
            token_threshold_to_create_proposal,
        } => {
            msg!("Instruction: {:?}", instruction);
            process_create_account_governance(
                program_id,
                accounts,
                &realm,
                &governed_account,
                vote_threshold,
                min_instruction_hold_up_time,
                max_voting_time,
                token_threshold_to_create_proposal,
            )
        }
        GovernanceInstruction::Execute => {
            msg!("Instruction: Execute");
            process_execute(program_id, accounts)
        }
        GovernanceInstruction::DepositSourceTokens {
            voting_token_amount,
        } => {
            msg!("Instruction: Deposit Source Tokens");
            process_deposit_source_tokens(program_id, accounts, voting_token_amount)
        }
        GovernanceInstruction::WithdrawVotingTokens {
            voting_token_amount,
        } => {
            msg!("Instruction: Withdraw Voting Tokens");
            process_withdraw_voting_tokens(program_id, accounts, voting_token_amount)
        }

        GovernanceInstruction::CreateEmptyGovernanceVoteRecord => {
            msg!("Instruction: Create Empty Governance Voting Record");
            process_create_empty_governance_voting_record(program_id, accounts)
        }

        GovernanceInstruction::CreateProposal {
            description_link,
            name,
        } => {
            msg!("Instruction:CreateProposal");
            process_create_proposal(program_id, accounts, description_link, name)
        }

        GovernanceInstruction::CreateRealm { name } => {
            msg!("Instruction:CreateGovernanceRealm");
            process_create_realm(program_id, accounts, name)
        }

        GovernanceInstruction::DepositGoverningTokens {} => {
            msg!("Instruction:DepositGoverningTokens");
            process_deposit_governing_tokens(program_id, accounts)
        }

        GovernanceInstruction::WithdrawGoverningTokens {} => {
            msg!("Instruction:WithdrawGoverningTokens");
            process_withdraw_governing_tokens(program_id, accounts)
        }

        GovernanceInstruction::SetVoteAuthority {
            realm,
            governing_token_mint,
            vote_authority,
        } => {
            msg!("Instruction:SetVoteAuthority");
            process_set_vote_authority(
                program_id,
                accounts,
                &realm,
                &governing_token_mint,
                &vote_authority,
            )
        }
    }
}
