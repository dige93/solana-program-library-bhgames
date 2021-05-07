use super::enums::GovernanceAccountType;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::program_pack::IsInitialized;

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Proposal {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// bla
    pub description_link: String,
    /// UTF-8 encoded name of the proposal
    pub name: String,
}

impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.account_type != GovernanceAccountType::Uninitialized
    }
}
