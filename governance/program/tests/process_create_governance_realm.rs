#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    let root_governance_cookie = governance_test.with_governance_realm().await;

    // Assert
    let root_governance_account = governance_test
        .get_root_governnace_account(&root_governance_cookie.address)
        .await;

    assert_eq!(root_governance_cookie.name, root_governance_account.name);
    assert_eq!(
        root_governance_cookie.governance_mint,
        root_governance_account.governance_mint
    );
    assert_eq!(
        root_governance_cookie.council_mint,
        root_governance_account.council_mint
    );
}
