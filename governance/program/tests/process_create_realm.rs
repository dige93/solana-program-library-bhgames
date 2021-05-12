#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    let realm_cookie = governance_test.with_realm().await;

    // Assert
    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert_eq!(realm_cookie.name, realm_account.name);
    assert_eq!(realm_cookie.governance_mint, realm_account.governance_mint);
    assert_eq!(realm_cookie.council_mint, realm_account.council_mint);
}
