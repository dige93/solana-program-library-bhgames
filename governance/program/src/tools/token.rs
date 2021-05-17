//! General purpose token utility functions

use arrayref::array_ref;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

use crate::error::GovernanceError;

/// Creates and initializes SPL token account
pub fn create_spl_token_account<'a>(
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

/// Creates and initializes SPL token account with PDA using the provided PDA seeds
pub fn create_spl_token_account_signed<'a>(
    payer_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    token_account_address_seeds: &Vec<&[u8]>,
    token_mint_info: &AccountInfo<'a>,
    token_account_owner_info: &AccountInfo<'a>,
    program_id: &Pubkey,
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

    let (account_address, bump_seed) =
        Pubkey::find_program_address(&token_account_address_seeds[..], program_id);

    if account_address != *token_account_info.key {
        msg!(
            "Create SPL Token Account with PDA: {:?} was requested while PDA: {:?} was expected",
            token_account_info.key,
            account_address
        );
        return Err(ProgramError::InvalidSeeds);
    }

    let mut signers_seeds = token_account_address_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &create_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            system_info.clone(),
        ],
        &[&signers_seeds[..]],
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

/// Creates and initializes SPL Token Mint
pub fn create_spl_token_mint<'a>(
    payer_info: &AccountInfo<'a>,
    mint_account_info: &AccountInfo<'a>,
    mint_authority: &Pubkey,
    system_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let create_account_instruction = system_instruction::create_account(
        payer_info.key,
        mint_account_info.key,
        1.max(Rent::default().minimum_balance(spl_token::state::Mint::LEN)),
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    invoke(
        &create_account_instruction,
        &[
            payer_info.clone(),
            mint_account_info.clone(),
            system_info.clone(),
        ],
    )?;

    let initialize_mint_instruction = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_account_info.key,
        mint_authority,
        None,
        0,
    )?;

    invoke(
        &initialize_mint_instruction,
        &[
            mint_account_info.clone(),
            spl_token_info.clone(),
            rent_sysvar_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers SPL Tokens
pub fn transfer_spl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    invoke(
        &transfer_instruction,
        &[
            spl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers SPL Tokens from a token account owned by the provided PDA authority with seeds
pub fn transfer_spl_tokens_signed<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &Vec<&[u8]>,
    program_id: &Pubkey,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let (authority_address, bump_seed) =
        Pubkey::find_program_address(&authority_seeds[..], program_id);

    if authority_address != *authority_info.key {
        msg!(
                "Transfer SPL Token with Authority PDA: {:?} was requested while PDA: {:?} was expected",
                authority_info.key,
                authority_address
            );
        return Err(ProgramError::InvalidSeeds);
    }

    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    let mut signers_seeds = authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &transfer_instruction,
        &[
            spl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    Ok(())
}

/// Mints SPL Tokens using the provided PDA mint authority with seeds
pub fn mint_spl_tokens_signed<'a>(
    token_mint_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    mint_authority_info: &AccountInfo<'a>,
    mint_authority_seeds: &Vec<&[u8]>,
    program_id: &Pubkey,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let (mint_authority_address, bump_seed) =
        Pubkey::find_program_address(&mint_authority_seeds[..], program_id);

    if mint_authority_address != *mint_authority_info.key {
        msg!(
            "Mint SPL Token with Mint Authority PDA: {:?} was requested while PDA: {:?} was expected",
            mint_authority_info.key,
            mint_authority_address
        );
        return Err(ProgramError::InvalidSeeds);
    }

    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        token_mint_info.key,
        token_account_info.key,
        mint_authority_info.key,
        &[],
        amount,
    )?;

    let mut signers_seeds = mint_authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &mint_to_instruction,
        &[
            spl_token_info.clone(),
            token_mint_info.clone(),
            token_account_info.clone(),
            mint_authority_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    Ok(())
}

/// Creates SPL Token Mint and Token Account with 1 token minted
/// The Token Mint authority is PDA with the provided seeds
/// This Token setup is used to grant permission expressed via the SPL token possession
pub fn setup_spl_token_permission_scheme_signed<'a>(
    payer_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    token_account_owner_info: &AccountInfo<'a>,
    mint_account_info: &AccountInfo<'a>,
    mint_authority_info: &AccountInfo<'a>,
    mint_authority_seeds: &Vec<&[u8]>,
    program_id: &Pubkey,
    system_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    create_spl_token_mint(
        payer_info,
        mint_account_info,
        mint_authority_info.key,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    create_spl_token_account(
        payer_info,
        token_account_info,
        mint_account_info,
        token_account_owner_info,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    mint_spl_tokens_signed(
        mint_account_info,
        token_account_info,
        mint_authority_info,
        mint_authority_seeds,
        program_id,
        1,
        spl_token_info,
    )?;

    Ok(())
}

/// Computationally cheap method to get amount from a token account
/// It reads amount without deserializing full account data
pub fn get_amount_from_token_account(
    token_account_info: &AccountInfo,
) -> Result<u64, ProgramError> {
    if token_account_info.owner != &spl_token::id() {
        return Err(GovernanceError::InvalidTokenAccountOwnerError.into());
    }

    // TokeAccount layout:   mint(32), owner(32), amount(8)
    let data = token_account_info.try_borrow_data()?;
    let amount = array_ref![data, 64, 8];
    Ok(u64::from_le_bytes(*amount))
}

/// Computationally cheap method to get mint from a token account
/// It reads mint without deserializing full account data
pub fn get_mint_from_token_account(
    token_account_info: &AccountInfo,
) -> Result<Pubkey, ProgramError> {
    if token_account_info.owner != &spl_token::id() {
        return Err(GovernanceError::InvalidTokenAccountOwnerError.into());
    }

    // TokeAccount layout:   mint(32), owner(32), amount(8)
    let data = token_account_info.try_borrow_data().unwrap();
    let mint_data = array_ref![data, 0, 32];
    Ok(Pubkey::new_from_array(*mint_data))
}

/// Asserts the expected owner signed the current transaction and owns an SPL token for the expected mint
/// It's used to validate permission expressed using the SPL token scheme
pub fn assert_spl_token_owner_is_signer<'a>(
    token_account_info: &AccountInfo<'a>,
    expected_token_mint: &Pubkey,
    expected_token_owner_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let token_data =
        spl_token::state::Account::unpack_from_slice(&token_account_info.data.borrow())?;

    if token_data.amount == 0
        || !token_data.is_initialized()
        || token_data.owner != *expected_token_owner_info.key
        || !expected_token_owner_info.is_signer
        || token_data.mint != *expected_token_mint
        || token_account_info.owner != &spl_token::id()
    {
        return Err(GovernanceError::TokenOwnerMustSign.into());
    }

    Ok(())
}
