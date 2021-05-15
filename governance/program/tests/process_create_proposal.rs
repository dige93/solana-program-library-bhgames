#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_community_proposal_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let account_governance_cookie = governance_test
        .with_account_governance(&realm_cookie, &governed_account_cookie)
        .await;

    // Act
    let proposal_cookie = governance_test
        .with_community_proposal(&account_governance_cookie)
        .await;

    // Assert
    let proposal_account = governance_test
        .get_proposal_account(&proposal_cookie.address)
        .await;

    assert_eq!(proposal_cookie.account, proposal_account);

    let account_governance_account = governance_test
        .get_program_governance_account(&account_governance_cookie.address)
        .await;

    assert_eq!(1, account_governance_account.proposal_count);
}
