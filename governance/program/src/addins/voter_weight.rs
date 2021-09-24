//! VoterWeight Addin interface

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{clock::UnixTimestamp, instruction::Instruction, pubkey::Pubkey};

use crate::tools::account::AccountMaxSize;

/// VoterWeight account type
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum VoterWeightAccountType {
    /// Default uninitialized account state
    Uninitialized,

    /// Voter Weight Record
    VoterWeightRecord,
}

/// VoterWeight Record account
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct VoterWeightRecord {
    /// VoterWeight account type
    pub account_type: VoterWeightAccountType,

    /// The owner of the governing token and voter
    pub governing_token_owner: Pubkey,

    /// Voter's weight
    pub voter_weight: u64,

    /// The as of timestamp the voter weight is calculated for
    pub voter_weight_at: UnixTimestamp,

    /// When the voting weight expires
    /// It can be used for voter weight decaying with time
    pub voter_weight_expiry: Option<UnixTimestamp>,
}

impl AccountMaxSize for VoterWeightRecord {}

/// /// VoterWeight instruction
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum VoterWeightInstruction {
    /// Revises voter weight providing up to date voter weight
    ///
    /// 0. `[writable]` VoterWeightRecord
    /// 1. `[]` Token owner
    Revise {
        /// The time offset (in seconds) into the future for which the voter weight should be revised
        #[allow(dead_code)]
        time_offset: u64,
    },
}

/// Creates Revise instruction
pub fn revise(
    program_id: &Pubkey,
    // Accounts

    // Args
    time_offset: u64,
) -> Instruction {
    let accounts = vec![];

    let instruction = VoterWeightInstruction::Revise { time_offset };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
