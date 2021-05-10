#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let governed_program_cookie = governance_test.with_dummy_governed_program().await;
    let governance_cookie = governance_test
        .with_program_governance(&governed_program_cookie)
        .await;

    // Act
    let proposal_cookie = governance_test.with_proposal(&governance_cookie).await;

    // Assert
    let proposal_account = governance_test
        .get_proposal_account(&proposal_cookie.address)
        .await;

    assert_eq!(proposal_cookie.name, proposal_account.name);
    assert_eq!(
        proposal_cookie.description_link,
        proposal_account.description_link
    );
}
