#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;
use spl_governance::error::GovernanceError;

#[tokio::test]
async fn test_withdraw_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    let deposit_amount = 10;

    // Act
    governance_test
        .withdraw_governance_token_deposit(
            &governance_realm_cookie,
            &voter_record_cookie,
            Some(50 - deposit_amount),
        )
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(deposit_amount, voter_record.governance_token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(deposit_amount, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.governance_token_source)
        .await;

    assert_eq!(
        voter_record_cookie.governance_token_source_amount - deposit_amount,
        source_account.amount
    );
}

#[tokio::test]
async fn test_withdraw_all_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    // Act
    governance_test
        .withdraw_governance_token_deposit(&governance_realm_cookie, &voter_record_cookie, None)
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(0, voter_record.governance_token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(0, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.governance_token_source)
        .await;

    assert_eq!(
        voter_record_cookie.governance_token_source_amount,
        source_account.amount
    );
}

#[tokio::test]
async fn test_withdraw_all_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    // Act
    governance_test
        .withdraw_council_token_deposit(&governance_realm_cookie, &voter_record_cookie, None)
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(0, voter_record.council_token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(
            &governance_realm_cookie
                .council_token_holding_account
                .unwrap(),
        )
        .await;

    assert_eq!(0, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.council_token_source.unwrap())
        .await;

    assert_eq!(
        voter_record_cookie.council_token_source_amount,
        source_account.amount
    );
}

#[tokio::test]
async fn test_withdraw_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;
    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    let deposit_amount = 10;

    // Act
    governance_test
        .withdraw_council_token_deposit(
            &governance_realm_cookie,
            &voter_record_cookie,
            Some(50 - deposit_amount),
        )
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(deposit_amount, voter_record.council_token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(
            &governance_realm_cookie
                .council_token_holding_account
                .unwrap(),
        )
        .await;

    assert_eq!(deposit_amount, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.council_token_source.unwrap())
        .await;

    assert_eq!(
        voter_record_cookie.council_token_source_amount - deposit_amount,
        source_account.amount
    );
}

#[tokio::test]
async fn test_withdraw_more_governance_tokens_then_deposited_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    // Act
    let error = governance_test
        .withdraw_governance_token_deposit(&governance_realm_cookie, &voter_record_cookie, Some(60))
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        error,
        GovernanceError::CannotWithdrawMoreGoverningTokensThenDeposited.into()
    );
}

#[tokio::test]
async fn test_withdraw_more_council_tokens_then_deposited_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;
    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    // Act
    let error = governance_test
        .withdraw_council_token_deposit(&governance_realm_cookie, &voter_record_cookie, Some(60))
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        error,
        GovernanceError::CannotWithdrawMoreGoverningTokensThenDeposited.into()
    );
}
