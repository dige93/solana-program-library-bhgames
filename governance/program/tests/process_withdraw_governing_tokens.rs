#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_withdraw_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governance_realm_cookie = governance_test.with_governance_realm().await;
    let voter_record_cookie = governance_test
        .with_initial_governance_token_deposit(&governance_realm_cookie, Some(50))
        .await;

    // Act
    governance_test
        .withdraw_governance_token_deposit(&governance_realm_cookie, &voter_record_cookie, 50)
        .await;

    // Assert
}
