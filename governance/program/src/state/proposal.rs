use crate::{id, tools::account::deserialize_account, PROGRAM_AUTHORITY_SEED};

use super::enums::{GovernanceAccountType, GoverningTokenType, ProposalState};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

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

    /// Mint that creates signatory tokens of this Proposal
    /// If there are outstanding signatory tokens, then cannot leave draft state. Signatories must burn tokens (ie agree
    /// to move instruction to voting state) and bring mint to net 0 tokens outstanding. Each signatory gets 1 (serves as flag)
    pub signatory_mint: Pubkey,

    /// Admin ownership mint. One token is minted, can be used to grant admin status to a new person
    pub admin_mint: Pubkey,
}

impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::Proposal
    }
}

/// Deserializes account and checks owner program
pub fn deserialize_proposal(proposal_info: &AccountInfo) -> Result<Proposal, ProgramError> {
    deserialize_account::<Proposal>(proposal_info, &id())
}

/// Returns Proposal PDA seeds
pub fn get_proposal_address_seeds<'a>(
    account_governance: &'a Pubkey,
    name: &'a str,
) -> Vec<&'a [u8]> {
    vec![
        PROGRAM_AUTHORITY_SEED,
        account_governance.as_ref(),
        &name.as_bytes(),
    ]
}

/// Returns Proposal PDA address
pub fn get_proposal_address<'a>(account_governance: &'a Pubkey, name: &'a str) -> Pubkey {
    Pubkey::find_program_address(
        &get_proposal_address_seeds(&account_governance, &name)[..],
        &id(),
    )
    .0
}
