#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_withdraw_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_governance_realm().await;
    let voter_record_setup = governance_test
        .with_initial_governance_token_deposit(&root_governance_setup)
        .await;

    // Act
    governance_test
        .withdraw_governance_token_deposit(&root_governance_setup, &voter_record_setup, 50)
        .await;

    // Assert
}
