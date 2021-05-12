use crate::{
    state::{enums::Vote, voter_record::get_voter_record_address},
    tools::get_realm_address,
};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{
    bpf_loader_upgradeable,
    epoch_schedule::Slot,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::id;

/// Instructions supported by the Governance program.
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[allow(clippy::large_enum_variant)]
#[allow(dead_code)]
pub enum GovernanceInstruction {
    /// Initializes a new empty Proposal for Instructions that will be executed at various slots in the future in draft mode.
    /// Grants Admin token to caller.
    ///
    ///   0. `[writable]` Uninitialized Proposal state account .
    ///   1. `[writable]` Uninitialized Proposal account .
    ///   2. `[writable]` Initialized Governance account.
    ///   3. `[writable]` Initialized Signatory Mint account
    ///   4. `[writable]` Initialized Admin Mint account
    ///   5. `[writable]` Initialized Voting Mint account
    ///   6. `[writable]` Initialized Yes Voting Mint account
    ///   7. `[writable]` Initialized No Voting Mint account
    ///   8. `[writable]` Initialized Signatory Validation account
    ///   9. `[writable]` Initialized Admin Validation account
    ///   10. `[writable]` Initialized Voting Validation account
    ///   11. `[writable]` Initialized Destination account for first admin token
    ///   12. `[writable]` Initialized Destination account for first signatory token
    ///   13. `[writable]` Initialized Yes voting dump account
    ///   14. `[writable]` Initialized No voting dump account
    ///   15. `[writable]` Initialized source holding account
    ///   16. `[]` Source mint
    ///   17. `[]` Governance minting authority (pda with seed of Proposal  key)
    ///   18. '[]` Token program id
    ///   19. `[]` Rent sysvar
    InitProposal {
        /// UTF-8 encoded gist explaining proposal
        #[allow(dead_code)]
        description_link: String,
        /// UTF-8 encoded name of the proposal
        #[allow(dead_code)]
        name: String,
    },

    /// [Requires Admin token]
    /// Adds a signatory to the Proposal which means that this Proposal can't leave Draft state until yet another signatory burns
    /// their signatory token indicating they are satisfied with the instruction queue. They'll receive an signatory token
    /// as a result of this call that they can burn later.
    ///
    ///   0. `[writable]` Initialized new signatory account.
    ///   1. `[writable]` Initialized Signatory mint account.
    ///   2. `[writable]` Admin account.
    ///   3. `[writable]` Admin validation account.
    ///   5. `[writable]` Proposal state account.
    ///   6. `[]` Proposal account.
    ///   7. `[]` Transfer authority
    ///   8. `[]` Governance program mint authority (pda of seed with Proposal key)
    ///   9. '[]` Token program id.
    AddSignatory,

    /// [Requires Admin token]
    /// Removes a signer from the set.
    ///
    ///   0. `[writable]` Signatory account to remove token from.
    ///   1. `[writable]` Signatory mint account.
    ///   2. `[writable]` Admin account.
    ///   3. `[writable]` Admin validation account.
    ///   4. `[writable]` Proposal state account.
    ///   5. `[]` Proposal account.
    ///   6. `[]` Transfer authority
    ///   7. `[]` Governance program mint authority (pda of seed with Proposal key)
    ///   8. '[]` Token program id.
    RemoveSignatory,

    /// [Requires Signatory token]
    /// Adds a Transaction to the Proposal Max of 5 of any Transaction type. More than 5 will throw error.
    /// Creates a PDA using your authority to be used to later execute the instruction.
    /// This transaction needs to contain authority to execute the program.
    ///
    ///   0. `[writable]` Uninitialized Proposal Transaction account.
    ///   1. `[writable]` Proposal state account.
    ///   2. `[writable]` Signatory account
    ///   3. `[writable]` Signatory validation account.
    ///   4. `[]` Proposal account.
    ///   5. `[]` Governance account.
    ///   6. `[]` Transfer authority
    ///   7. `[]` Governance mint authority
    ///   8. `[]` Governance program account.
    ///   9. `[]` Token program account.
    AddCustomSingleSignerTransaction {
        /// Slot during which this will run
        #[allow(dead_code)]
        delay_slots: u64,
        /// Instruction
        #[allow(dead_code)]
        instruction: Vec<u8>,
        /// Position in transaction array
        #[allow(dead_code)]
        position: u8,
        /// Point in instruction array where 0 padding begins - inclusive, index should be where actual instruction ends, not where 0s begin
        #[allow(dead_code)]
        instruction_end_index: u16,
    },

    /// [Requires Signatory token]
    /// Remove Transaction from the Proposal.
    ///
    ///   0. `[writable]` Proposal state account.
    ///   1. `[writable]` Proposal Transaction account.
    ///   2. `[writable]` Signatory account
    ///   3. `[writable]` Signatory validation account.
    ///   5. `[]` Proposal.
    ///   6. `[]` Transfer Authority.
    ///   7. `[]` Governance mint authority (pda of seed Proposal  key)
    ///   9. `[]` Token program account.
    RemoveTransaction,

    /// [Requires Signatory token]
    /// Update Transaction slot in the Proposal. Useful during reset periods.
    ///
    ///   1. `[writable]` Proposal Transaction account.
    ///   2. `[writable]` Signatory account
    ///   3. `[writable]` Signatory validation account.
    ///   4. `[]` Proposal state account.
    ///   5. `[]` Proposal account.
    ///   6. `[]` Transfer authority.
    ///   7. `[]` Governance mint authority (pda with seed of Proposal key)
    ///   8. `[]` Token program account.
    UpdateTransactionDelaySlots {
        /// On what slot this transaction slot will now run
        #[allow(dead_code)]
        delay_slots: u64,
    },

    /// [Requires Admin token]
    /// Cancels Proposal by moving it into Cancelled state.
    ///
    ///   0. `[writable]` Proposal state account pub key.
    ///   1. `[writable]` Admin account
    ///   2. `[writable]` Admin validation account.
    ///   3. `[]` Proposal account pub key.
    ///   4. `[]` Transfer authority.
    ///   5. `[]` Governance mint authority (pda with seed of Proposal key)
    ///   6. `[]` Token program account.
    CancelProposal,

    /// [Requires Signatory token]
    /// Burns signatory token, indicating you approve of moving this Proposal from Draft state to Voting state.
    /// The last Signatory token to be burned moves the state to Voting.
    ///
    ///   0. `[writable]` Proposal state account pub key.
    ///   1. `[writable]` Signatory account
    ///   2. `[writable]` Signatory mint account.
    ///   3. `[]` Proposal account pub key.
    ///   4. `[]` Transfer authority
    ///   5. `[]` Governance mint authority (pda of seed Proposal key)ß
    ///   7. `[]` Token program account.
    ///   8. `[]` Clock sysvar.
    SignProposal,

    /// [Requires Voting tokens]
    /// Burns voting tokens, indicating you approve and/or disapprove of running this set of transactions. If you tip the consensus,
    /// then the transactions can begin to be run at their time slots when people click execute. You are then given yes and/or no tokens.
    ///
    ///   0. `[writable]` Governance voting record account.
    ///                   Can be uninitialized or initialized(if already used once in this proposal)
    ///                   Must have address with PDA having seed tuple [Governance acct key, proposal key, your voting account key]
    ///   1. `[writable]` Proposal state account.
    ///   2. `[writable]` Your Voting account.
    ///   3. `[writable]` Your Yes-Voting account.
    ///   4. `[writable]` Your No-Voting account.
    ///   5. `[writable]` Voting mint account.
    ///   6. `[writable]` Yes Voting mint account.
    ///   7. `[writable]` No Voting mint account.
    ///   8. `[]` Source mint account
    ///   9. `[]` Proposal account.
    ///   10. `[]` Governance account.
    ///   12. `[]` Transfer authority
    ///   13. `[]` Governance program mint authority (pda of seed Proposal key)
    ///   14. `[]` Token program account.
    ///   15. `[]` Clock sysvar.
    Vote {
        /// Casted vote
        #[allow(dead_code)]
        vote: Vote,
    },

    /// Executes a command in the Proposal
    ///
    ///   0. `[writable]` Transaction account you wish to execute.
    ///   1. `[writable]` Proposal state account.
    ///   2. `[]` Program being invoked account
    ///   3. `[]` Proposal account.
    ///   4. `[]` Governance account
    ///   5. `[]` Governance program account pub key.
    ///   6. `[]` Clock sysvar.
    ///   7+ Any extra accounts that are part of the instruction, in order
    Execute,

    /// [Requires tokens of the Governance mint or Council mint depending on type of Proposal]
    /// Deposits voting tokens to be used during the voting process in a Proposal.
    /// These tokens are removed from your account and can be returned by withdrawing
    /// them from the Proposal (but then you will miss the vote.)
    ///
    ///   0. `[writable]` Governance voting record account. See Vote docs for more detail.
    ///   1. `[writable]` Initialized Voting account to hold your received voting tokens.
    ///   2. `[writable]` User token account to deposit tokens from.
    ///   3. `[writable]` Source holding account for Proposal that will accept the tokens in escrow.
    ///   4. `[writable]` Voting mint account.
    ///   5. `[]` Proposal account.
    ///   6. `[]` Transfer authority
    ///   7. `[]` Governance program mint authority (pda with seed of Proposal key)
    ///   8. `[]` Token program account.
    DepositSourceTokens {
        /// How many voting tokens to deposit
        #[allow(dead_code)]
        voting_token_amount: u64,
    },

    /// [Requires voting tokens]
    /// Withdraws voting tokens.
    ///
    ///   0. `[writable]` Governance voting record account. See Vote docs for more detail.
    ///   1. `[writable]` Initialized Voting account from which to remove your voting tokens.
    ///   2. `[writable]` Initialized Yes Voting account from which to remove your voting tokens.
    ///   3. `[writable]` Initialized No Voting account from which to remove your voting tokens.
    ///   4. `[writable]` User token account that you wish your actual tokens to be returned to.
    ///   5. `[writable]` Source holding account owned by the Governance that will has the actual tokens in escrow.
    ///   6. `[writable]` Initialized Yes Voting dump account owned by Proposal to which to send your voting tokens.
    ///   7. `[writable]` Initialized No Voting dump account owned by Proposal to which to send your voting tokens.
    ///   8. `[writable]` Voting mint account.
    ///   9. `[writable]` Yes Voting mint account.
    ///   10. `[writable]` No Voting mint account.
    ///   11. `[]` Proposal state account.
    ///   12. `[]` Proposal account.
    ///   13. `[]` Transfer authority
    ///   14. `[]` Governance program mint authority (pda of seed Proposal key)
    ///   15. `[]` Token program account.
    WithdrawVotingTokens {
        /// How many voting tokens to withdrawal
        #[allow(dead_code)]
        voting_token_amount: u64,
    },

    /// Creates Program Governance account
    ///
    ///   0. `[writable]` Governance account. The account pubkey needs to be set to program-derived address (PDA) with the following seeds:
    ///           1) 'governance' const prefix
    ///           2) Governed Program address
    ///   1. `[]` Account of the Program governed by this Governance account
    ///   2. `[writable]` Program Data account of the Program governed by this Governance account
    ///   3. `[signer]` Current Upgrade Authority account of the Program governed by this Governance account
    ///   4. `[]` Governance mint that this Governance uses
    ///   5. `[signer]` Payer
    ///   6. `[]` System account
    ///   7. `[]` Bpf_upgrade_loader account
    ///   8. `[]` Council mint that this Governance uses [Optional]
    CreateProgramGovernance {
        /// Voting threshold in % required to tip the vote
        /// It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
        #[allow(dead_code)]
        vote_threshold: u8,

        /// Minimum waiting time in slots for an instruction to be executed after proposal is voted on
        #[allow(dead_code)]
        min_instruction_hold_up_time: Slot,

        /// Time limit in slots for proposal to be open to voting
        #[allow(dead_code)]
        max_voting_time: Slot,
        // Minimum % of tokens for a governance token owner to be able to create proposal
        // It's the percentage of tokens out of the entire pool of governance tokens eligible to vote
        // TODO: Add field
        //token_threshold_to_create_proposal: u8,
    },

    ///   0. `[]` Governance vote record key. Needs to be set with pubkey set to PDA with seeds of the
    ///           program account key, proposal key, your voting account key.
    ///   1. `[]` Proposal key
    ///   2. `[]` Your voting account
    ///   3. `[]` Payer
    ///   5. `[]` System account.
    CreateEmptyGovernanceVoteRecord,

    /// Creates Proposal Account
    CreateProposal {
        /// UTF-8 encoded gist explaining proposal
        #[allow(dead_code)]
        description_link: String,
        /// UTF-8 encoded name of the proposal
        #[allow(dead_code)]
        name: String,
    },

    /// Creates Governance Realm which aggregates governances for given Governance Mint and optional Council Mint.
    ///
    /// 1. `[writable]` Governance Realm account.
    /// 2. `[]` Governance Token Mint.
    /// 3. `[writable, signer]` Governances Token Holding account.
    /// 4. `[signer]` Payer.
    /// 5. `[]` System.
    /// 6. `[]` SPL Token.
    /// 7. `[]` Sysvar Rent.
    /// 8. `[]` Council Token mint - optional.
    /// 9. `[writable, signer]` Council Token Holding account - optional.
    CreateRealm {
        /// UTF-8 encoded Governance Realm name
        #[allow(dead_code)]
        name: String,
    },

    DepositGoverningTokens {},

    /// Sets vote authority for the given Realm and Governing Token Mint (Governance or Council)
    /// The vote authority would have voting rights and could vote on behalf of the Governing Token Owner
    ///
    /// 0. `[signer]` Governing Token Owner
    /// 1. `[writable]` Voter Record
    SetVoteAuthority {
        #[allow(dead_code)]
        /// Governance Realm the new vote authority is set for
        realm: Pubkey,

        #[allow(dead_code)]
        /// Governing Token Mint the vote authority is granted over
        governing_token_mint: Pubkey,

        #[allow(dead_code)]
        /// New vote authority
        vote_authority: Pubkey,
    },

    WithdrawGoverningTokens {},
}

pub fn set_vote_authority(
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
    vote_authority: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let vote_record_address =
        get_voter_record_address(realm, governing_token_mint, governing_token_owner);

    let accounts = vec![
        AccountMeta::new_readonly(*governing_token_owner, true),
        AccountMeta::new(vote_record_address, false),
    ];

    let instruction = GovernanceInstruction::SetVoteAuthority {
        realm: *realm,
        governing_token_mint: *governing_token_mint,
        vote_authority: *vote_authority,
    };

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}

/// Creates CreateRealm instruction
pub fn create_realm(
    name: String,
    governance_token_mint: &Pubkey,
    governance_token_holding: &Pubkey,
    payer: &Pubkey,
    council_token_mint: Option<Pubkey>,
    council_token_holding: Option<Pubkey>,
) -> Result<Instruction, ProgramError> {
    let realm_address = get_realm_address(&name);

    let mut accounts = vec![
        AccountMeta::new(realm_address, false),
        AccountMeta::new_readonly(*governance_token_mint, false),
        AccountMeta::new(*governance_token_holding, true),
        AccountMeta::new_readonly(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    if let Some(council_mint) = council_token_mint {
        accounts.push(AccountMeta::new_readonly(council_mint, false));
        accounts.push(AccountMeta::new(council_token_holding.unwrap(), true));
    }

    let instruction = GovernanceInstruction::CreateRealm { name };

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}

/// Creates WithdrawGoverningTokens instruction
pub fn withdraw_governing_tokens(
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
    governing_token_holding: &Pubkey,
    governing_token_destination: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let vote_record_address =
        get_voter_record_address(realm, governing_token_mint, governing_token_owner);

    let accounts = vec![
        AccountMeta::new_readonly(*realm, false),
        AccountMeta::new_readonly(*governing_token_mint, false),
        AccountMeta::new(*governing_token_holding, false),
        AccountMeta::new(*governing_token_destination, false),
        AccountMeta::new_readonly(*governing_token_owner, true),
        AccountMeta::new(vote_record_address, false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let instruction = GovernanceInstruction::WithdrawGoverningTokens {};

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}

/// Creates DepositGoverningTokens instruction
pub fn deposit_governing_tokens(
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
    governing_token_holding: &Pubkey,
    governing_token_source: &Pubkey,
    governing_token_owner: &Pubkey,
    payer: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let vote_record_address =
        get_voter_record_address(realm, governing_token_mint, governing_token_owner);

    let accounts = vec![
        AccountMeta::new_readonly(*realm, false),
        AccountMeta::new(*governing_token_holding, false),
        AccountMeta::new(*governing_token_source, false),
        AccountMeta::new_readonly(*governing_token_owner, true),
        AccountMeta::new(vote_record_address, false),
        AccountMeta::new_readonly(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let instruction = GovernanceInstruction::DepositGoverningTokens {};

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}

/// Creates CreateProgramGovernance instruction
pub fn create_program_governance(
    program_governance: &Pubkey,
    governed_program: &Pubkey,
    governed_program_data: &Pubkey,
    governed_program_upgrade_authority: &Pubkey,
    governance_mint: &Pubkey,
    payer: &Pubkey,
    council_mint: &Option<Pubkey>,
    vote_threshold: u8,
    min_instruction_hold_up_time: u64,
    max_voting_time: u64,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*program_governance, false),
        AccountMeta::new_readonly(*governed_program, false),
        AccountMeta::new(*governed_program_data, false),
        AccountMeta::new_readonly(*governed_program_upgrade_authority, true),
        AccountMeta::new_readonly(*governance_mint, false),
        AccountMeta::new_readonly(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(bpf_loader_upgradeable::id(), false),
    ];

    if let Some(council_mint_key) = council_mint {
        accounts.push(AccountMeta::new_readonly(*council_mint_key, false));
    }

    let instruction = GovernanceInstruction::CreateProgramGovernance {
        vote_threshold,
        min_instruction_hold_up_time,
        max_voting_time,
    };

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}

/// Creates CreateProposal instruction
pub fn create_proposal(
    description_link: String,
    name: String,

    proposal_address: &Pubkey,
    governance_address: &Pubkey,
    payer: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*proposal_address, true),
        AccountMeta::new(*governance_address, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let instruction = GovernanceInstruction::CreateProposal {
        description_link,
        name,
    };

    Ok(Instruction {
        program_id: id(),
        accounts,
        data: instruction.try_to_vec().unwrap(),
    })
}
