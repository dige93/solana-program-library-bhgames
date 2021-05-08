#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_deposited_to_empty() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_root_governance().await;

    // Act
    let voter_record_setup = governance_test
        .with_governance_token_deposit(root_governance_setup)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_setup.address)
        .await;

    assert_eq!(
        voter_record_setup.governance_token_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_setup.council_token_amount,
        voter_record.council_token_amount
    );
}
