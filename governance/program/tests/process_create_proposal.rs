#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;
use solana_sdk::signature::Signer;

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

    let admin_account = governance_test
        .get_token_account(&proposal_cookie.admin_token_account)
        .await;

    assert_eq!(proposal_cookie.proposal_owner.pubkey(), admin_account.owner);

    let signatory_account = governance_test
        .get_token_account(&proposal_cookie.signatory_token_account)
        .await;

    assert_eq!(1, signatory_account.amount);
    assert_eq!(
        proposal_cookie.proposal_owner.pubkey(),
        signatory_account.owner
    );

    let account_governance_account = governance_test
        .get_program_governance_account(&account_governance_cookie.address)
        .await;

    assert_eq!(1, account_governance_account.proposal_count);
}

#[tokio::test]
async fn test_multiple_proposals_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let account_governance_cookie = governance_test
        .with_account_governance(&realm_cookie, &governed_account_cookie)
        .await;

    // Act
    let community_proposal_cookie = governance_test
        .with_community_proposal(&account_governance_cookie)
        .await;

    let council_proposal_cookie = governance_test
        .with_council_proposal(&account_governance_cookie)
        .await;

    // Assert
    let community_proposal_account = governance_test
        .get_proposal_account(&community_proposal_cookie.address)
        .await;

    assert_eq!(
        community_proposal_cookie.account,
        community_proposal_account
    );

    let council_proposal_account = governance_test
        .get_proposal_account(&council_proposal_cookie.address)
        .await;

    assert_eq!(council_proposal_cookie.account, council_proposal_account);

    let account_governance_account = governance_test
        .get_program_governance_account(&account_governance_cookie.address)
        .await;

    assert_eq!(2, account_governance_account.proposal_count);
}
