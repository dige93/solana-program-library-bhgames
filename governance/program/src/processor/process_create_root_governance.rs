//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    state::{enums::GovernanceAccountType, root_governance::RootGovernance},
    tools::{
        accounts::create_and_serialize_account_signed, get_root_governance_address_seeds,
        token::create_token_account,
    },
};

/// process_create_root_governance
pub fn process_create_root_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let root_governance_info = next_account_info(account_info_iter)?; // 1
    let governance_token_mint_info = next_account_info(account_info_iter)?; // 2
    let governance_token_holding_info = next_account_info(account_info_iter)?; // 3
    let payer_info = next_account_info(account_info_iter)?; // 4
    let system_info = next_account_info(account_info_iter)?; // 5
    let spl_token_info = next_account_info(account_info_iter)?; // 6
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 7

    let root_governance_data = RootGovernance {
        account_type: GovernanceAccountType::RootGovernance,
        governance_mint: *governance_token_mint_info.key,
        name: name.clone(),
    };

    create_and_serialize_account_signed::<RootGovernance>(
        payer_info,
        &root_governance_info,
        &root_governance_data,
        get_root_governance_address_seeds(&name),
        program_id,
        system_info,
    )?;

    create_token_account(
        payer_info,
        governance_token_holding_info,
        governance_token_mint_info,
        root_governance_info,
        system_info,
        spl_token_info,
        rent_sysvar_info,
    )?;

    if let Ok(council_mint_info) = next_account_info(account_info_iter) {
        // 8
        let council_token_holding_info = next_account_info(account_info_iter)?; //9

        create_token_account(
            payer_info,
            council_token_holding_info,
            council_mint_info,
            root_governance_info,
            system_info,
            spl_token_info,
            rent_sysvar_info,
        )?;
    }

    Ok(())
}
