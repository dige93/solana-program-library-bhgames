use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

#[derive(Debug)]
pub struct GovernedProgramCookie {
    pub address: Pubkey,
    pub upgrade_authority: Keypair,
    pub data_address: Pubkey,
}

#[derive(Debug)]
pub struct ProgramGovernanceCookie {
    pub address: Pubkey,
    pub governance_mint: Pubkey,
    pub council_mint: Option<Pubkey>,
    pub vote_threshold: u8,
    pub min_instruction_hold_up_time: u64,
    pub max_voting_time: u64,
}
#[derive(Debug)]
pub struct ProposalCookie {
    pub address: Pubkey,
    pub description_link: String,
    /// UTF-8 encoded name of the proposal
    pub name: String,
}

#[derive(Debug)]
pub struct GovernanceRealmCookie {
    pub address: Pubkey,

    /// UTF-8 encoded name of the proposal
    pub name: String,

    pub governance_mint: Pubkey,

    pub governance_mint_authority: Keypair,

    pub governance_token_holding_account: Pubkey,

    pub council_mint: Option<Pubkey>,

    pub council_mint_authority: Option<Keypair>,

    pub council_token_holding_account: Option<Pubkey>,
}

#[derive(Debug)]
pub struct VoterRecordCookie {
    pub address: Pubkey,

    pub governance_token_deposit_amount: u64,

    pub governance_token_source: Pubkey,

    pub council_token_deposit_amount: u64,

    pub council_token_source: Option<Pubkey>,
}
