#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    let governance_realm_cookie = governance_test.with_governance_realm().await;

    // Assert
    let root_governance_account = governance_test
        .get_root_governnace_account(&governance_realm_cookie.address)
        .await;

    assert_eq!(governance_realm_cookie.name, root_governance_account.name);
    assert_eq!(
        governance_realm_cookie.governance_mint,
        root_governance_account.governance_mint
    );
    assert_eq!(
        governance_realm_cookie.council_mint,
        root_governance_account.council_mint
    );
}
