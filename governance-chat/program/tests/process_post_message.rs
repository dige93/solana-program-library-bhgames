#![cfg(feature = "test-bpf")]

use program_test::GovernanceChatProgramTest;
use solana_program_test::tokio;

mod program_test;

#[tokio::test]
async fn test_post_message() {
    // Arrange
    let mut governance_chat_test = GovernanceChatProgramTest::start_new().await;

    let message_cookie = governance_chat_test.with_message().await;
}
