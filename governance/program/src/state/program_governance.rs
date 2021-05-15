use crate::{state::enums::GovernanceAccountType, PROGRAM_AUTHORITY_SEED};

use solana_program::{
    epoch_schedule::Slot,
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Governance Account
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ProgramGovernance {
    /// Account type
    pub account_type: GovernanceAccountType,

    /// Voting threshold in % required to tip the vote
    /// It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
    pub vote_threshold: u8,

    /// Minimum % of tokens for a governance token owner to be able to create a proposal
    /// It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
    // TODO: Add Field
    // pub token_threshold_to_create_proposal: u8,

    /// Minimum waiting time in slots for an instruction to be executed after proposal is voted on
    pub min_instruction_hold_up_time: Slot,

    /// Governance mint
    pub governance_mint: Pubkey,

    /// Council mint
    pub council_mint: Option<Pubkey>,

    /// Program ID that is governed by this Governance
    pub program: Pubkey,

    /// Time limit in slots for proposal to be open for voting
    pub max_voting_time: u64,

    /// Running count of proposals
    pub proposal_count: u32,
}

impl Sealed for ProgramGovernance {}

impl IsInitialized for ProgramGovernance {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::ProgramGovernance
    }
}

pub fn get_program_governance_address_seeds(governed_program_address: &Pubkey) -> Vec<&[u8]> {
    vec![PROGRAM_AUTHORITY_SEED, &governed_program_address.as_ref()]
}
