#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

use spl_governance::state::enums::ProposalState;

#[tokio::test]
async fn test_proposal_cancelled() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let account_governance_cookie = governance_test
        .with_account_governance(&realm_cookie, &governed_account_cookie)
        .await;

    let proposal_cookie = governance_test
        .with_community_proposal(&account_governance_cookie)
        .await;

    // Act
    governance_test.cancel_proposal(&proposal_cookie).await;

    // Assert
    let proposal_account = governance_test
        .get_proposal_account(&proposal_cookie.address)
        .await;

    assert_eq!(ProposalState::Cancelled, proposal_account.state);
}
