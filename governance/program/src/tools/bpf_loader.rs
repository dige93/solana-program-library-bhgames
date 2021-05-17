//! General purpose bpf_loader utility functions

use bincode::deserialize;
use solana_program::{
    account_info::AccountInfo, bpf_loader_upgradeable, program_error::ProgramError, pubkey::Pubkey,
};

use serde_derive::{Deserialize, Serialize};

use crate::error::GovernanceError;

/// Upgradeable loader account states.
/// Note: The struct is taken as is from solana-sdk which doesn't support bpf and can't be referenced from a program
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum UpgradeableLoaderState {
    /// Account is not initialized.
    Uninitialized,
    /// A Buffer account.
    Buffer {
        /// Authority address
        authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
    /// An Program account.
    Program {
        /// Address of the ProgramData account.
        programdata_address: Pubkey,
    },
    /// A ProgramData account.
    ProgramData {
        /// Slot that the program was last modified.
        slot: u64,
        /// Address of the Program's upgrade authority.
        upgrade_authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
}

/// Checks whether the expected program upgrade authority is the current upgrade authority of the program
/// If it's not then it asserts the current program upgrade authority  is a signer of the transaction
pub fn assert_program_upgrade_authority(
    expected_upgrade_authority: &Pubkey,
    program_address: &Pubkey,
    program_data_info: &AccountInfo,
    program_upgrade_authority_info: &AccountInfo,
) -> Result<(), ProgramError> {
    if program_data_info.owner != &bpf_loader_upgradeable::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (program_data_account_key, _) =
        Pubkey::find_program_address(&[program_address.as_ref()], &bpf_loader_upgradeable::id());

    if program_data_account_key != *program_data_info.key {
        return Err(GovernanceError::InvalidProgramDataAccountKey.into());
    }

    let upgrade_authority = match deserialize(&program_data_info.data.borrow())
        .map_err(|_| GovernanceError::InvalidProgramDataAccountData)?
    {
        UpgradeableLoaderState::ProgramData {
            slot: _,
            upgrade_authority_address,
        } => upgrade_authority_address,
        _ => None,
    };

    match upgrade_authority {
        Some(upgrade_authority) => {
            if upgrade_authority != *expected_upgrade_authority {
                if upgrade_authority != *program_upgrade_authority_info.key {
                    return Err(GovernanceError::InvalidUpgradeAuthority.into());
                }
                if !program_upgrade_authority_info.is_signer {
                    return Err(GovernanceError::UpgradeAuthorityMustSign.into());
                }
            }
            Ok(())
        }
        None => Err(GovernanceError::ProgramNotUpgradable.into()),
    }
}
