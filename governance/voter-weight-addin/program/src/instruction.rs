//! Program instructions

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Instructions supported by the VoterWeightInstruction addin program
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[allow(clippy::large_enum_variant)]
pub enum VoterWeightAddinInstruction {
    /// Revises voter weight providing up to date voter weight
    ///
    /// 0. `[writable]` VoterWeightRecord
    /// 1. `[]` Token owner
    Revise {
        /// The time offset (in seconds) into the future for which the voter weight should be revised
        #[allow(dead_code)]
        time_offset: u64,
    },

    /// Deposits given amount
    /// VoterWeightRecord
    Deposit {
        /// The deposit amount
        #[allow(dead_code)]
        amount: u64,
    },
}
