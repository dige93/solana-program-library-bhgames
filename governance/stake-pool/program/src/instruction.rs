//! Program instructions

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Instructions supported by the GovernanceStakePool program
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[allow(clippy::large_enum_variant)]
pub enum GovernanceStakePoolInstruction {
    /// Deposit instruction
    Deposit,
}
