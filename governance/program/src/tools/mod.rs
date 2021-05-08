//! Governance utility functions
#![allow(missing_docs)]

pub mod accounts;

use solana_program::pubkey::Pubkey;

use crate::{id, PROGRAM_AUTHORITY_SEED};

pub fn get_root_governance_address_seeds(name: &String) -> Vec<&[u8]> {
    vec![PROGRAM_AUTHORITY_SEED, &name.as_bytes()]
}

pub fn get_root_governance_address(name: &String) -> Pubkey {
    Pubkey::find_program_address(&get_root_governance_address_seeds(&name)[..], &id()).0
}
