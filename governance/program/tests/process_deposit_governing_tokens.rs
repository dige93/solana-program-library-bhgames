#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_deposited_initial_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    // Act
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(10))
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(
        voter_record_cookie.governance_token_deposit_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_cookie.council_token_deposit_amount,
        voter_record.council_token_amount
    );

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.governance_token_source)
        .await;
    assert_eq!(
        voter_record_cookie.governance_token_source_amount
            - voter_record_cookie.governance_token_deposit_amount,
        source_account.amount
    );

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(voter_record.governance_token_amount, holding_account.amount);
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
        voter_record_cookie.governance_token_deposit_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_cookie.council_token_deposit_amount,
        voter_record.council_token_amount
    );

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.council_token_source.unwrap())
        .await;
    assert_eq!(100, source_account.amount);

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(voter_record.council_token_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_subsequent_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    let governance_token_holding_account = governance_realm_cookie.governance_token_holding_account;

    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(10))
        .await;

    let deposit_amount = 10;
    let total_deposit_amount = voter_record_cookie.governance_token_deposit_amount + deposit_amount;

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

    assert_eq!(total_deposit_amount, voter_record.governance_token_amount);

    let holding_account = governance_test
        .get_token_account(&governance_token_holding_account)
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

    let deposit_amount = 10;
    let total_deposit_amount = voter_record_cookie.council_token_deposit_amount + deposit_amount;

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

    assert_eq!(total_deposit_amount, voter_record.council_token_amount);

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(total_deposit_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_all_initial_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    // Act
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, None)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(
        voter_record_cookie.governance_token_deposit_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_cookie.council_token_deposit_amount,
        voter_record.council_token_amount
    );

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.governance_token_source)
        .await;
    assert_eq!(0, source_account.amount);

    let holding_account = governance_test
        .get_token_account(&governance_realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(voter_record.governance_token_amount, holding_account.amount);
}
