use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use spl_governance::state::{
    account_governance::AccountGovernance, proposal::Proposal, realm::Realm,
    voter_record::VoterRecord,
};

#[derive(Debug)]
pub struct GovernedProgramCookie {
    pub address: Pubkey,
    pub upgrade_authority: Keypair,
    pub data_address: Pubkey,
}

#[derive(Debug)]
pub struct GovernedAccountCookie {
    pub address: Pubkey,
}

#[derive(Debug)]
pub struct AccountGovernanceCookie {
    pub address: Pubkey,
    pub account: AccountGovernance,
}
#[derive(Debug)]
pub struct ProposalCookie {
    pub address: Pubkey,
    pub account: Proposal,
    pub proposal_owner: Keypair,

    pub admin_token_account: Pubkey,
    pub signatory_token_account: Pubkey,
}

#[derive(Debug)]
pub struct RealmCookie {
    pub address: Pubkey,

    pub account: Realm,

    pub governance_mint_authority: Keypair,

    pub governance_token_holding_account: Pubkey,

    pub council_mint_authority: Option<Keypair>,

    pub council_token_holding_account: Option<Pubkey>,
}

#[derive(Debug)]
pub struct VoterRecordCookie {
    pub address: Pubkey,

    pub account: VoterRecord,

    pub token_source: Pubkey,

    pub token_source_amount: u64,

    pub token_owner: Keypair,

    pub vote_authority: Keypair,
}
