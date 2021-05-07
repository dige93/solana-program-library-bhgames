#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    // Act
    governance_test.with_proposal().await;
}
