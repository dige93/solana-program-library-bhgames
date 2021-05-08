//! General purpose account utility functions

use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, msg, program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_instruction::create_account,
};

/// Creates a new account and serializes data into it using the provided seeds to make signed CPI call
/// Note: This functions also checks the provided account Program Derived Address matches the supplied seeds
pub fn create_and_serialize_account_signed<'a, T: BorshSerialize>(
    payer_info: &AccountInfo<'a>,
    account_info: &AccountInfo<'a>,
    account_data: &T,
    account_address_seeds: Vec<&[u8]>,
    account_owner: &Pubkey,
    system_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    // Get PDA and assert it's the same as the requested account address
    let (account_address, bump_seed) =
        Pubkey::find_program_address(&account_address_seeds[..], account_owner);

    if account_address != *account_info.key {
        msg!(
            "Create account with Program Derived Address: {:?} was requested while Address: {:?} was expected",
            account_info.key,
            account_address
        );
        return Err(ProgramError::InvalidSeeds);
    }
    let serialized_data = account_data.try_to_vec()?;

    let create_account_instruction = create_account(
        payer_info.key,
        account_info.key,
        Rent::default().minimum_balance(serialized_data.len()),
        serialized_data.len() as u64,
        account_owner,
    );

    let mut signers_seeds = account_address_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &create_account_instruction,
        &[
            payer_info.clone(),
            account_info.clone(),
            system_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    account_info
        .data
        .borrow_mut()
        .copy_from_slice(&serialized_data);

    Ok(())
}
