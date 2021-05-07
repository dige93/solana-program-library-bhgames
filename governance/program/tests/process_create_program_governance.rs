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
        program_governance_setup.min_instruction_hold_up_time,
        program_governance_account.min_instruction_hold_up_time
    );
    assert_eq!(
        program_governance_setup.max_voting_time,
        program_governance_account.max_voting_time
    );

    assert_eq!(
        program_governance_setup.governance_mint,
        program_governance_account.governance_mint
    );
    assert_eq!(true, program_governance_account.council_mint.is_none());
}
