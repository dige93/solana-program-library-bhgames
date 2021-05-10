#![cfg(feature = "test-bpf")]

use solana_program_test::*;

mod program_test;

use program_test::*;

#[tokio::test]
async fn test_created() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;
    let governed_program_cookie = governance_test.with_governed_program().await;

    // Act
    let program_governance_cookie = governance_test
        .with_program_governance(&governed_program_cookie)
        .await;

    // Assert
    let program_governance_account = governance_test
        .get_program_governance_account(&program_governance_cookie.address)
        .await;

    assert_eq!(
        program_governance_cookie.vote_threshold,
        program_governance_account.vote_threshold
    );
    assert_eq!(
        program_governance_cookie.min_instruction_hold_up_time,
        program_governance_account.min_instruction_hold_up_time
    );
    assert_eq!(
        program_governance_cookie.max_voting_time,
        program_governance_account.max_voting_time
    );

    assert_eq!(
        program_governance_cookie.governance_mint,
        program_governance_account.governance_mint
    );
    assert_eq!(true, program_governance_account.council_mint.is_none());
}
