#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_deposited_initial_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_root_governance().await;

    let governance_token_holding_account = root_governance_setup.governance_token_holding_account;

    // Act
    let voter_record_setup = governance_test
        .with_initial_governance_token_deposit(&root_governance_setup)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_setup.address)
        .await;

    assert_eq!(
        voter_record_setup.governance_token_deposit_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_setup.council_token_deposit_amount,
        voter_record.council_token_amount
    );

    let source_account = governance_test
        .get_token_account(&voter_record_setup.governance_token_source)
        .await;
    assert_eq!(100, source_account.amount);

    let holding_account = governance_test
        .get_token_account(&governance_token_holding_account)
        .await;

    assert_eq!(voter_record.governance_token_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_initial_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_root_governance().await;

    let council_token_holding_account =
        root_governance_setup.council_token_holding_account.unwrap();

    // Act
    let voter_record_setup = governance_test
        .with_initial_council_token_deposit(&root_governance_setup)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_setup.address)
        .await;

    assert_eq!(
        voter_record_setup.governance_token_deposit_amount,
        voter_record.governance_token_amount
    );

    assert_eq!(
        voter_record_setup.council_token_deposit_amount,
        voter_record.council_token_amount
    );

    let source_account = governance_test
        .get_token_account(&voter_record_setup.council_token_source.unwrap())
        .await;
    assert_eq!(100, source_account.amount);

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(voter_record.council_token_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_subsequent_governance_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_root_governance().await;

    let governance_token_holding_account = root_governance_setup.governance_token_holding_account;

    let voter_record_setup = governance_test
        .with_initial_governance_token_deposit(&root_governance_setup)
        .await;

    let deposit_amount = 10;
    let total_deposit_amount = voter_record_setup.governance_token_deposit_amount + deposit_amount;

    // Act
    governance_test
        .with_governance_token_deposit(&root_governance_setup, &voter_record_setup, deposit_amount)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_setup.address)
        .await;

    assert_eq!(total_deposit_amount, voter_record.governance_token_amount);

    let holding_account = governance_test
        .get_token_account(&governance_token_holding_account)
        .await;

    assert_eq!(total_deposit_amount, holding_account.amount);
}

#[tokio::test]
async fn test_deposited_subsequent_council_tokens() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let root_governance_setup = governance_test.with_root_governance().await;

    let council_token_holding_account =
        root_governance_setup.council_token_holding_account.unwrap();

    let voter_record_setup = governance_test
        .with_initial_council_token_deposit(&root_governance_setup)
        .await;

    let deposit_amount = 10;
    let total_deposit_amount = voter_record_setup.council_token_deposit_amount + deposit_amount;

    // Act
    governance_test
        .with_council_token_deposit(&root_governance_setup, &voter_record_setup, deposit_amount)
        .await;

    // Assert
    let voter_record = governance_test
        .get_voter_record_account(&voter_record_setup.address)
        .await;

    assert_eq!(total_deposit_amount, voter_record.council_token_amount);

    let holding_account = governance_test
        .get_token_account(&council_token_holding_account)
        .await;

    assert_eq!(total_deposit_amount, holding_account.amount);
}
