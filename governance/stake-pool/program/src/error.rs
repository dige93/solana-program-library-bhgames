//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the GovernanceStakePool program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum GovernanceStakePoolError {}

impl PrintProgramError for GovernanceStakePoolError {
    fn print<E>(&self) {
        msg!("GOVERNANCE-STAKE-POOL-ERROR: {}", &self.to_string());
    }
}

impl From<GovernanceStakePoolError> for ProgramError {
    fn from(e: GovernanceStakePoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GovernanceStakePoolError {
    fn type_of() -> &'static str {
        "Governance Stake Pool Error"
    }
}
