#![cfg(feature = "test-bpf")]

use program_test::GovernanceChatProgramTest;
use solana_program_test::tokio;
use spl_governance_test_sdk::GovernanceProgramTest;

mod program_test;

#[tokio::test]
async fn test_post_message() {
    // Arrange
    let mut governance_chat_test = GovernanceChatProgramTest::start_new().await;

    governance_chat_test.governance.with_realm().await;

    let message_cookie = governance_chat_test.with_message().await;
}

#[tokio::test]
async fn test_create_realm() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    let realm_cookie = governance_test.with_realm().await;

    // Assert
    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert_eq!(realm_cookie.account, realm_account);
}
