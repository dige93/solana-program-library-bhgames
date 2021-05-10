//! RootGovernance Account

use super::enums::GovernanceAccountType;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct GovernanceRealm {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// Governance mint
    pub governance_mint: Pubkey,

    /// Council mint
    pub council_mint: Option<Pubkey>,

    /// Governance Realm name
    pub name: String,
}

impl IsInitialized for GovernanceRealm {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::GovernanceRealm
    }
}
