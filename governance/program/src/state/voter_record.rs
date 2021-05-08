//! VoterRecord Account

use super::enums::GovernanceAccountType;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::program_pack::IsInitialized;

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct VoterRecord {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    pub governance_token_amount: u64,

    pub council_token_amount: Option<u64>,

    pub active_votes_count: u8,
}

impl IsInitialized for VoterRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::VoterRecord
    }
}
