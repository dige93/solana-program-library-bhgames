/// Defines all Governance accounts types
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// GovernanceAccountType
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAccountType {
    /// 0 - Default uninitialized account state
    Uninitialized,

    /// 1 - Program Governance account
    ProgramGovernance,

    /// 2 - Proposal account for Governance account. A single Governance account can have multiple Proposal accounts
    ProposalOld,

    /// 3 - Proposal voting state account. Every Proposal account has exactly one ProposalState account
    ProposalState,

    /// 4 - Vote record account for a given Proposal.  Proposal can have 0..n voting records
    VoteRecord,

    /// 5 Custom Single Signer Transaction account which holds instructions to execute for Proposal
    CustomSingleSignerTransaction,

    /// 6
    Proposal,
}

impl Default for GovernanceAccountType {
    fn default() -> Self {
        GovernanceAccountType::Uninitialized
    }
}

/// What state a Proposal is in
#[derive(Clone, Debug, PartialEq)]
pub enum ProposalStateStatus {
    /// Draft
    Draft,

    /// Taking votes
    Voting,

    /// Votes complete, in execution phase
    Executing,

    /// Completed, can be rebooted
    Completed,

    /// Canceled
    Canceled,

    /// Defeated
    Defeated,
}

impl Default for ProposalStateStatus {
    fn default() -> Self {
        ProposalStateStatus::Draft
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
