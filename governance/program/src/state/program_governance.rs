use crate::{
    state::enums::GovernanceAccountType,
    utils::{pack_option_key, unpack_option_key},
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    epoch_schedule::Slot,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// max name length
pub const GOVERNANCE_NAME_LENGTH: usize = 32;
/// Governance Account
#[derive(Clone, Debug, Default, PartialEq)]
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
        self.account_type != GovernanceAccountType::Uninitialized
    }
}

/// Len of Governance
pub const GOVERNANCE_LEN: usize = 1 + 1 + 8 + 32 + 33 + 32 + 8 + 4 + 295;

impl Pack for ProgramGovernance {
    const LEN: usize = 1 + 1 + 8 + 32 + 33 + 32 + 8 + 4 + 295;
    /// Unpacks a byte buffer into Governance account data
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, GOVERNANCE_LEN];
        // TODO think up better way than txn_* usage here - new to rust
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            account_type_value,
            vote_threshold,
            minimum_slot_waiting_period,
            governance_mint,
            council_mint_option,
            program,
            time_limit,
            proposal_count,
            _padding,
        ) = array_refs![input, 1, 1, 8, 32, 33, 32, 8, 4, 295];
        let account_type = u8::from_le_bytes(*account_type_value);
        let vote_threshold = u8::from_le_bytes(*vote_threshold);
        let minimum_slot_waiting_period = u64::from_le_bytes(*minimum_slot_waiting_period);
        let time_limit = u64::from_le_bytes(*time_limit);
        let proposal_count = u32::from_le_bytes(*proposal_count);

        let account_type = match account_type {
            0 => GovernanceAccountType::Uninitialized,
            1 => GovernanceAccountType::ProgramGovernance,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Self {
            account_type,
            vote_threshold,

            min_instruction_hold_up_time: minimum_slot_waiting_period,
            governance_mint: Pubkey::new_from_array(*governance_mint),

            council_mint: unpack_option_key(council_mint_option)?,

            program: Pubkey::new_from_array(*program),
            max_voting_time: time_limit,
            proposal_count,
        })
    }

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, GOVERNANCE_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            account_type_value,
            vote_threshold,
            minimum_slot_waiting_period,
            governance_mint,
            council_mint_option,
            program,
            time_limit,
            proposal_count,
            _padding,
        ) = mut_array_refs![output, 1, 1, 8, 32, 33, 32, 8, 4, 295];
        *account_type_value = match self.account_type {
            GovernanceAccountType::Uninitialized => 0_u8,
            GovernanceAccountType::ProgramGovernance => 1_u8,
            _ => panic!("Account type was invalid"),
        }
        .to_le_bytes();

        *vote_threshold = self.vote_threshold.to_le_bytes();

        *minimum_slot_waiting_period = self.min_instruction_hold_up_time.to_le_bytes();
        governance_mint.copy_from_slice(self.governance_mint.as_ref());

        pack_option_key(self.council_mint, council_mint_option);

        program.copy_from_slice(self.program.as_ref());
        *time_limit = self.max_voting_time.to_le_bytes();

        *proposal_count = self.proposal_count.to_le_bytes();
    }

    fn get_packed_len() -> usize {
        Self::LEN
    }

    fn unpack(input: &[u8]) -> Result<Self, ProgramError>
    where
        Self: IsInitialized,
    {
        let value = Self::unpack_unchecked(input)?;
        if value.is_initialized() {
            Ok(value)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    }

    fn unpack_unchecked(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Self::unpack_from_slice(input)
    }

    fn pack(src: Self, dst: &mut [u8]) -> Result<(), ProgramError> {
        if dst.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        src.pack_into_slice(dst);
        Ok(())
    }
}
