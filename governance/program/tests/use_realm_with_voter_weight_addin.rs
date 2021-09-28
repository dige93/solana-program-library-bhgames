#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_create_realm_with_voter_weight_addin() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_voter_weight_addin().await;

    // Act

    let realm_cookie = governance_test.with_realm().await;

    // Assert

    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert!(realm_account.config.use_voter_weight_addin);

    // TODO: Check addins
}

#[tokio::test]
async fn test_create_governance_with_voter_weight_addin() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_voter_weight_addin().await;
    let governed_account_cookie = governance_test.with_governed_account().await;

    let realm_cookie = governance_test.with_realm().await;

    let mut token_owner_record_cookie =
        governance_test.with_token_owner_record(&realm_cookie).await;

    let voter_weight_cookie = governance_test
        .with_voter_weight_addin_deposit(&token_owner_record_cookie)
        .await
        .unwrap();

    token_owner_record_cookie.voter_weight = Some(voter_weight_cookie);

    // Act
    let _account_governance_cookie = governance_test
        .with_account_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // // Assert
}
