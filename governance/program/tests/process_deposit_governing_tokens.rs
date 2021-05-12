#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;
use solana_sdk::signature::Signer;
use spl_governance::state::enums::GoverningTokenType;

#[tokio::test]
async fn test_deposited_initial_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    // Act
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie)
        .await;

    // Assert

    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(
        voter_record_cookie.token_deposit_amount,
        voter_record.token_deposit_amount
    );

    assert_eq!(
        voter_record_cookie.token_owner.pubkey(),
        voter_record.token_owner
    );

    assert_eq!(
        voter_record_cookie.vote_authority.pubkey(),
        voter_record.vote_authority
    );

    assert_eq!(governance_realm_cookie.address, voter_record.realm);

    assert_eq!(0, voter_record.active_votes_count);

    assert_eq!(GoverningTokenType::Governance, voter_record.token_type);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.token_source)
        .await;

    assert_eq!(
        voter_record_cookie.token_source_amount - voter_record_cookie.token_deposit_amount,
        source_account.amount
    );

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(voter_record.token_deposit_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_initial_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let council_token_holding_account = governance_realm_cookie
        .council_token_holding_account
        .unwrap();

    // Act
    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&governance_realm_cookie)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(
        voter_record_cookie.token_deposit_amount,
        voter_record.token_deposit_amount
    );

    assert_eq!(
        voter_record_cookie.token_owner.pubkey(),
        voter_record.token_owner
    );

    assert_eq!(
        voter_record_cookie.vote_authority.pubkey(),
        voter_record.vote_authority
    );

    assert_eq!(0, voter_record.active_votes_count);

    assert_eq!(GoverningTokenType::Council, voter_record.token_type);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.token_source)
        .await;

    assert_eq!(
        voter_record_cookie.token_source_amount - voter_record_cookie.token_deposit_amount,
        source_account.amount
    );

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(voter_record.token_deposit_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_subsequent_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie)
        .await;

    let deposit_amount = 5;
    let total_deposit_amount = voter_record_cookie.token_deposit_amount + deposit_amount;

    // Act
    governance_test
        .with_governance_token_deposit(
            &governance_realm_cookie,
            &voter_record_cookie,
            deposit_amount,
        )
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(total_deposit_amount, voter_record.token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(total_deposit_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_subsequent_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let council_token_holding_account = governance_realm_cookie
        .council_token_holding_account
        .unwrap();

    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&governance_realm_cookie)
        .await;

    let deposit_amount = 5;
    let total_deposit_amount = voter_record_cookie.token_deposit_amount + deposit_amount;

    // Act
    governance_test
        .with_council_token_deposit(
            &governance_realm_cookie,
            &voter_record_cookie,
            deposit_amount,
        )
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(total_deposit_amount, voter_record.token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(total_deposit_amount, holding_account.amount);
}
