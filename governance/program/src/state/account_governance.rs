use crate::{id, state::enums::GovernanceAccountType};

use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Account Governance
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct AccountGovernance {
    /// Account type
    pub account_type: GovernanceAccountType,

    /// Governance Realm
    pub realm: Pubkey,

    /// Voting threshold in % required to tip the vote
    /// It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
    pub vote_threshold: u8,

    /// Minimum % of tokens for a governance token owner to be able to create a proposal
    /// It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
    pub token_threshold_to_create_proposal: u8,

    /// Minimum waiting time in slots for an instruction to be executed after proposal is voted on
    pub min_instruction_hold_up_time: u64,

    /// Account governed by this Governance. It can be for example Program account, Mint account or Token Account
    pub governed_account: Pubkey,

    /// Time limit in slots for proposal to be open for voting
    pub max_voting_time: u64,

    /// Running count of proposals
    pub proposal_count: u32,
}

impl Sealed for AccountGovernance {}

impl IsInitialized for AccountGovernance {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::AccountGovernance
    }
}

pub fn get_program_governance_address_seeds<'a>(
    realm: &'a Pubkey,
    governed_program: &'a Pubkey,
) -> Vec<&'a [u8]> {
    vec![
        b"program-governance",
        &realm.as_ref(),
        &governed_program.as_ref(),
    ]
}

pub fn get_program_governance_address<'a>(
    realm: &'a Pubkey,
    governed_program: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_program_governance_address_seeds(realm, governed_program)[..],
        &id(),
    )
    .0
}

pub fn get_account_governance_address_seeds<'a>(
    realm: &'a Pubkey,
    governed_account: &'a Pubkey,
) -> Vec<&'a [u8]> {
    vec![
        b"account-governance",
        &realm.as_ref(),
        &governed_account.as_ref(),
    ]
}

pub fn get_account_governance_address<'a>(
    realm: &'a Pubkey,
    governed_account: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_account_governance_address_seeds(realm, governed_account)[..],
        &id(),
    )
    .0
}
