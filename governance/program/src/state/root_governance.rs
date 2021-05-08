//! RootGovernance Account

use super::enums::GovernanceAccountType;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::program_pack::IsInitialized;

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RootGovernance {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// Governance name
    pub name: String,
}

impl IsInitialized for RootGovernance {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::RootGovernance
    }
}
