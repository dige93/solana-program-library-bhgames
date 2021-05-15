use super::enums::{GovernanceAccountType, GoverningTokenType, ProposalState};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

/// Governance Proposal
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Proposal {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// Account Governance the Proposal belongs to
    pub account_governance: Pubkey,

    /// Indicates which Governing Token is used to vote on the Proposal
    /// Whether the general Community token owners or the Council tokens owners vote on this Proposal
    pub governing_token_type: GoverningTokenType,

    /// Current state of the Proposal
    pub state: ProposalState,

    /// Link to Proposal's description
    pub description_link: String,

    /// UTF-8 encoded Proposal name
    pub name: String,
}

impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::Proposal
    }
}
