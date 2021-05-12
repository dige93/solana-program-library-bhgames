#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_withdraw_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let realm_cookie = governance_test.with_realm().await;

    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&realm_cookie)
        .await;

    // Act
    governance_test
        .withdraw_governance_tokens(&realm_cookie, &voter_record_cookie)
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(0, voter_record.token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&realm_cookie.governance_token_holding_account)
        .await;

    assert_eq!(0, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.token_source)
        .await;

    assert_eq!(
        voter_record_cookie.token_source_amount,
        source_account.amount
    );
}

#[tokio::test]
async fn test_withdraw_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let realm_cookie = governance_test.with_realm().await;

    let voter_record_cookie = governance_test
        .with_initial_council_token_deposit(&realm_cookie)
        .await;

    // Act
    governance_test
        .withdraw_council_tokens(&realm_cookie, &voter_record_cookie)
        .await
        .unwrap();

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_cookie.address)
        .await;

    assert_eq!(0, voter_record.token_deposit_amount);

    let holding_account = governance_test
        .get_token_account(&realm_cookie.council_token_holding_account.unwrap())
        .await;

    assert_eq!(0, holding_account.amount);

    let source_account = governance_test
        .get_token_account(&voter_record_cookie.token_source)
        .await;

    assert_eq!(
        voter_record_cookie.token_source_amount,
        source_account.amount
    );
}
