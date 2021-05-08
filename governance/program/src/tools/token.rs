//! General purpose token utility functions

use solana_program::{
    account_info::AccountInfo, program::invoke, program_error::ProgramError, program_pack::Pack,
    rent::Rent, system_instruction,
};

pub fn create_token_account<'a>(
    payer_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    token_mint_info: &AccountInfo<'a>,
    token_account_owner_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let create_account_instruction = system_instruction::create_account(
        payer_info.key,
        token_account_info.key,
        1.max(Rent::default().minimum_balance(spl_token::state::Account::get_packed_len())),
        spl_token::state::Account::get_packed_len() as u64,
        &spl_token::id(),
    );

    invoke(
        &create_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            system_info.clone(),
        ],
    )?;

    let initialize_account_instruction = spl_token::instruction::initialize_account(
        &spl_token::id(),
        token_account_info.key,
        token_mint_info.key,
        token_account_owner_info.key,
    )?;

    invoke(
        &initialize_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            token_account_owner_info.clone(),
            token_mint_info.clone(),
            spl_token_info.clone(),
            rent_sysvar_info.clone(),
        ],
    )?;

    Ok(())
}
