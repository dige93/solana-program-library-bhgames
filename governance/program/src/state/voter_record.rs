//! VoterRecord Account

use crate::{error::GovernanceError, id, tools::account::deserialize_account};

use super::enums::GovernanceAccountType;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct VoterRecord {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    pub voter: Pubkey,

    pub governance_token_amount: u64,

    pub council_token_amount: u64,

    pub active_votes_count: u8,
}

impl IsInitialized for VoterRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::VoterRecord
    }
}

pub fn deserialize_voter_record(
    voter_record_info: &AccountInfo,
    voter_info: &AccountInfo,
) -> Result<VoterRecord, ProgramError> {
    let voter_record_data = deserialize_account::<VoterRecord>(voter_record_info, &id())?;

    if voter_record_data.voter != *voter_info.key {
        return Err(GovernanceError::InvalidVoterAccount.into());
    }

    Ok(voter_record_data)
}
