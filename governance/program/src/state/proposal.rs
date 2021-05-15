use crate::{id, PROGRAM_AUTHORITY_SEED};

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

pub fn get_proposal_address_seeds<'a>(
    account_governance: &'a Pubkey,
    name: &'a String,
) -> Vec<&'a [u8]> {
    vec![
        PROGRAM_AUTHORITY_SEED,
        account_governance.as_ref(),
        &name.as_bytes(),
    ]
}

pub fn get_proposal_address<'a>(account_governance: &'a Pubkey, name: &'a String) -> Pubkey {
    Pubkey::find_program_address(
        &get_proposal_address_seeds(&account_governance, &name)[..],
        &id(),
    )
    .0
}
