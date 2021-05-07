use super::{
    enums::GovernanceAccountType,
    proposal_state::{DESC_SIZE, NAME_SIZE},
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Governance Proposal
#[derive(Clone)]
pub struct Proposal {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// bla
    pub description_link: [u8; DESC_SIZE],
    /// UTF-8 encoded name of the proposal
    // TODO: Change to String
    pub name: [u8; NAME_SIZE],
}

impl Sealed for Proposal {}
impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.account_type != GovernanceAccountType::Uninitialized
    }
}

const PROPOSAL_LEN: usize = 1 + DESC_SIZE + NAME_SIZE;
impl Pack for Proposal {
    const LEN: usize = 1 + DESC_SIZE + NAME_SIZE;
    /// Unpacks a byte buffer into a Proposal account data
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, PROPOSAL_LEN];
        // TODO think up better way than txn_* usage here - new to rust
        #[allow(clippy::ptr_offset_with_cast)]
        let (account_type_value, description_link, name) =
            array_refs![input, 1, DESC_SIZE, NAME_SIZE];
        let account_type = u8::from_le_bytes(*account_type_value);

        let account_type = match account_type {
            0 => GovernanceAccountType::Uninitialized,
            6 => GovernanceAccountType::Proposal,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(Self {
            account_type,
            name: *name,
            description_link: *description_link,
        })
    }

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, PROPOSAL_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (account_type_value, description_link, name) =
            mut_array_refs![output, 1, DESC_SIZE, NAME_SIZE];

        *account_type_value = match self.account_type {
            GovernanceAccountType::Uninitialized => 0_u8,
            GovernanceAccountType::Proposal => 6_u8,
            _ => panic!("Account type was invalid"),
        }
        .to_le_bytes();

        description_link.copy_from_slice(self.description_link.as_ref());
        name.copy_from_slice(self.name.as_ref());
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
