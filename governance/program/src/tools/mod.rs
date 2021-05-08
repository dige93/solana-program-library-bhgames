//! Program utility functions
#![allow(missing_docs)]
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::GovernanceError, id, PROGRAM_AUTHORITY_SEED};

pub fn get_root_governance_address(name: &String) -> Result<Pubkey, ProgramError> {
    Ok(get_root_governance_address_with_bump_seed(name, None)?.0)
}

pub fn get_root_governance_address_with_bump_seed(
    name: &String,
    expected_address: Option<&Pubkey>,
) -> Result<(Pubkey, u8), ProgramError> {
    let (address, bump_seed) =
        Pubkey::find_program_address(&[PROGRAM_AUTHORITY_SEED, &name.as_bytes()], &id());

    if let Some(expected_address) = expected_address {
        if expected_address != &address {
            msg!(
                "Got {:?} RootGovernance PDA while {:?} was expected",
                address,
                expected_address
            );
            return Err(GovernanceError::InvalidProgramDerivedAddress.into());
        }
    }

    Ok((address, bump_seed))
}
