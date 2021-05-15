/// Defines all Governance accounts types
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// GovernanceAccountType
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAccountType {
    /// 0 - Default uninitialized account state
    Uninitialized,

    ///
    Realm,

    ///
    VoterRecord,

    /// 1 - Account Governance account
    AccountGovernance,

    /// 2 - Proposal account for Governance account. A single Governance account can have multiple Proposal accounts
    ProposalOld,

    /// 3 - Proposal voting state account. Every Proposal account has exactly one ProposalState account
    ProposalState,

    /// 4 - Vote record account for a given Proposal.  Proposal can have 0..n voting records
    ProposalVoteRecord,

    /// 5 Custom Single Signer Transaction account which holds instructions to execute for Proposal
    SingleSignerTransaction,

    /// 6
    Proposal,
}

impl Default for GovernanceAccountType {
    fn default() -> Self {
        GovernanceAccountType::Uninitialized
    }
}

/// What state a Proposal is in
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ProposalState {
    /// Draft - Proposal enters Draft state when it's created
    Draft,

    /// Signing - The Proposal is being signed by Signatories. Proposal enters the state when first Signatory Sings and leaves it when last Signatory signs
    Signing,

    /// Taking votes
    Voting,

    /// Voting ended with success
    Succeeded,

    /// Voting completed and now instructions are being execute. Proposal enter this state when first instruction is executed and leaves when the last instruction is executed
    Executing,

    /// Completed
    Completed,

    /// Cancelled
    Cancelled,

    /// Defeated
    Defeated,
}

impl Default for ProposalState {
    fn default() -> Self {
        ProposalState::Draft
    }
}

/// Vote  with number of votes
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum Vote {
    /// Yes vote
    Yes(u64),

    /// No vote
    No(u64),
}

/// Governing Token type
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GoverningTokenType {
    /// Community token
    Community,
    /// Council token
    Council,
}
