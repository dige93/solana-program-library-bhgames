#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;
use spl_governance::addins::voter_weight::revise;

#[tokio::test]
async fn test_create_realm_with_voter_weight_addin() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_voter_weight_addin().await;

    let _realm_cookie = governance_test.with_realm().await;

    // Act
    let revise_ix = revise(&governance_test.voter_weight_addin_id.unwrap(), 100);

    governance_test
        .bench
        .process_transaction(&[revise_ix], None)
        .await
        .unwrap();

    // // Assert
}
