use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use crate::state::enums::GovernanceAccountType;

/// Governance Proposal
#[derive(Clone)]
pub struct ProposalOld {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// Governance account the Proposal belongs to
    pub governance: Pubkey,

    /// Proposal State account
    pub state: Pubkey,

    /// Mint that creates signatory tokens of this Proposal
    /// If there are outstanding signatory tokens, then cannot leave draft state. Signatories must burn tokens (ie agree
    /// to move instruction to voting state) and bring mint to net 0 tokens outstanding. Each signatory gets 1 (serves as flag)
    pub signatory_mint: Pubkey,

    /// Admin ownership mint. One token is minted, can be used to grant admin status to a new person.
    pub admin_mint: Pubkey,

    /// Mint that creates voting tokens of this Proposal
    pub vote_mint: Pubkey,

    /// Mint that creates evidence of voting YES via token creation
    pub yes_vote_mint: Pubkey,

    /// Mint that creates evidence of voting NO via token creation
    pub no_vote_mint: Pubkey,

    /// Used to validate signatory tokens in a round trip transfer
    pub signatory_validation: Pubkey,

    /// Used to validate admin tokens in a round trip transfer
    pub admin_validation: Pubkey,

    /// Used to validate voting tokens in a round trip transfer
    pub vote_validation: Pubkey,

    /// Source Token Holding account
    pub source_holding: Pubkey,

    /// Source Mint - either governance or council mint from Governance
    pub source_mint: Pubkey,
}

impl Sealed for ProposalOld {}
impl IsInitialized for ProposalOld {
    fn is_initialized(&self) -> bool {
        self.account_type != GovernanceAccountType::Uninitialized
    }
}

const PROPOSAL_LEN: usize = 1 + 32 * 12 + 300;
impl Pack for ProposalOld {
    const LEN: usize = 1 + 32 * 12 + 300;
    /// Unpacks a byte buffer into a Proposal account data
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, PROPOSAL_LEN];
        // TODO think up better way than txn_* usage here - new to rust
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            account_type_value,
            governance,
            state,
            signatory_mint,
            admin_mint,
            vote_mint,
            yes_voting_mint,
            no_voting_mint,
            source_mint,
            signatory_validation,
            admin_validation,
            vote_validation,
            source_holding,
            _padding,
        ) = array_refs![input, 1, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 300];
        let account_type = u8::from_le_bytes(*account_type_value);

        let account_type = match account_type {
            0 => GovernanceAccountType::Uninitialized,
            2 => GovernanceAccountType::ProposalOld,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(Self {
            account_type,
            governance: Pubkey::new_from_array(*governance),
            state: Pubkey::new_from_array(*state),
            signatory_mint: Pubkey::new_from_array(*signatory_mint),
            admin_mint: Pubkey::new_from_array(*admin_mint),
            vote_mint: Pubkey::new_from_array(*vote_mint),
            yes_vote_mint: Pubkey::new_from_array(*yes_voting_mint),
            no_vote_mint: Pubkey::new_from_array(*no_voting_mint),
            source_mint: Pubkey::new_from_array(*source_mint),
            signatory_validation: Pubkey::new_from_array(*signatory_validation),
            admin_validation: Pubkey::new_from_array(*admin_validation),
            vote_validation: Pubkey::new_from_array(*vote_validation),
            source_holding: Pubkey::new_from_array(*source_holding),
        })
    }

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, PROPOSAL_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            account_type_value,
            governance,
            state,
            signatory_mint,
            admin_mint,
            voting_mint,
            yes_voting_mint,
            no_voting_mint,
            source_mint,
            signatory_validation,
            admin_validation,
            vote_validation,
            source_holding,
            _padding,
        ) = mut_array_refs![output, 1, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 300];

        *account_type_value = match self.account_type {
            GovernanceAccountType::Uninitialized => 0_u8,
            GovernanceAccountType::ProposalOld => 2_u8,
            _ => panic!("Account type was invalid"),
        }
        .to_le_bytes();

        governance.copy_from_slice(self.governance.as_ref());
        state.copy_from_slice(self.state.as_ref());
        signatory_mint.copy_from_slice(self.signatory_mint.as_ref());
        admin_mint.copy_from_slice(self.admin_mint.as_ref());
        voting_mint.copy_from_slice(self.vote_mint.as_ref());
        yes_voting_mint.copy_from_slice(self.yes_vote_mint.as_ref());
        no_voting_mint.copy_from_slice(self.no_vote_mint.as_ref());
        source_mint.copy_from_slice(self.source_mint.as_ref());
        signatory_validation.copy_from_slice(self.signatory_validation.as_ref());
        admin_validation.copy_from_slice(self.admin_validation.as_ref());
        vote_validation.copy_from_slice(self.vote_validation.as_ref());
        source_holding.copy_from_slice(self.source_holding.as_ref());
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
