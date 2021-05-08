#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let governed_program_setup = governance_test.with_dummy_governed_program().await;
    let governance_setup = governance_test
        .with_program_governance(&governed_program_setup)
        .await;

    // Act
    let proposal_setup = governance_test.with_proposal(&governance_setup).await;

    // Assert
    let proposal_account = governance_test
        .get_proposal_account(&proposal_setup.address)
        .await;

    assert_eq!(proposal_setup.name, proposal_account.name);
    assert_eq!(
        proposal_setup.description_link,
        proposal_account.description_link
    );
}
