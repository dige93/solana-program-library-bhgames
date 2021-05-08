#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    let root_governance_setup = governance_test.with_root_governance().await;

    // Assert
    let root_governance_account = governance_test
        .get_root_governnace_account(&root_governance_setup.address)
        .await;

    assert_eq!(root_governance_setup.name, root_governance_account.name);
    assert_eq!(
        root_governance_setup.governance_mint,
        root_governance_account.governance_mint
    );
}
