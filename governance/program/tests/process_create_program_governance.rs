#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governed_program_setup = governance_test.with_governed_program().await;

    // Act
    let program_governance_setup = governance_test
        .with_program_governance(&governed_program_setup)
        .await;

    // Assert
    let program_governance_account = governance_test
        .get_program_governance_account(&program_governance_setup.address)
        .await;

    assert_eq!(
        program_governance_setup.vote_threshold,
        program_governance_account.vote_threshold
    );
    assert_eq!(
        program_governance_setup.minimum_slot_waiting_period,
        program_governance_account.minimum_slot_waiting_period
    );
    assert_eq!(program_governance_setup.time_limit, program_governance_account.time_limit);
    assert_eq!(program_governance_setup.name, program_governance_account.name);
    assert_eq!(
        program_governance_setup.governance_mint,
        program_governance_account.governance_mint
    );
    assert_eq!(true, program_governance_account.council_mint.is_none());
}

#[tokio::test]
async fn test_create_dummy_account() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    governance_test.with_dummy_account().await;
}
