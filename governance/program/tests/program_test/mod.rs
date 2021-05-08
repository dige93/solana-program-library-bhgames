use std::{env, fs::File, io::Read, path::PathBuf};

use borsh::BorshDeserialize;
use solana_program::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    instruction::Instruction,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};
use solana_program_test::ProgramTest;
use solana_program_test::*;

use solana_sdk::{
    hash::Hash,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_governance::{
    id,
    instruction::{create_governance, create_proposal, create_root_governance},
    processor::process_instruction,
    state::{
        program_governance::ProgramGovernance, proposal::Proposal, root_governance::RootGovernance,
    },
    tools::get_root_governance_address,
    PROGRAM_AUTHORITY_SEED,
};

#[derive(Debug)]
pub struct GovernedProgramSetup {
    pub address: Pubkey,
    pub upgrade_authority: Keypair,
    pub data_address: Pubkey,
}

#[derive(Debug)]
pub struct ProgramGovernanceSetup {
    pub address: Pubkey,
    pub governance_mint: Pubkey,
    pub council_mint: Option<Pubkey>,
    pub vote_threshold: u8,
    pub min_instruction_hold_up_time: u64,
    pub max_voting_time: u64,
}
#[derive(Debug)]
pub struct ProposalSetup {
    pub address: Pubkey,
    /// bla
    pub description_link: String,
    /// UTF-8 encoded name of the proposal
    pub name: String,
}

#[derive(Debug)]
pub struct RootGovernanceSetup {
    pub address: Pubkey,

    /// UTF-8 encoded name of the proposal
    pub name: String,

    pub governance_mint: Pubkey,

    pub council_mint: Option<Pubkey>,
}

pub struct GovernanceProgramTest {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub rent: Rent,
}

impl GovernanceProgramTest {
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::new(
            "spl_governance",
            spl_governance::id(),
            processor!(process_instruction),
        );

        program_test.add_program(
            "solana_bpf_loader_upgradeable_program",
            bpf_loader_upgradeable::id(),
            Some(solana_bpf_loader_program::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let rent = banks_client.get_rent().await.unwrap();

        Self {
            banks_client,
            payer,
            recent_blockhash,
            rent,
        }
    }

    async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: Option<&[&Keypair]>,
    ) {
        let mut transaction =
            Transaction::new_with_payer(&instructions, Some(&self.payer.pubkey()));

        let mut all_signers = vec![&self.payer];

        if let Some(signers) = signers {
            all_signers.extend_from_slice(signers);
        }

        transaction.sign(&all_signers, self.recent_blockhash);

        self.banks_client
            .process_transaction(transaction)
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn with_governed_program(&mut self) -> GovernedProgramSetup {
        let program_address_keypair = Keypair::new();
        let program_buffer_keypair = Keypair::new();
        let program_upgrade_authority_keypair = Keypair::new();

        let (program_data_address, _) = Pubkey::find_program_address(
            &[program_address_keypair.pubkey().as_ref()],
            &bpf_loader_upgradeable::id(),
        );

        // Load solana_bpf_rust_upgradeable program taken from solana test programs
        let program_data = read_governed_program("solana_bpf_rust_upgradeable");

        let program_buffer_rent = self
            .rent
            .minimum_balance(UpgradeableLoaderState::programdata_len(program_data.len()).unwrap());

        let mut instructions = bpf_loader_upgradeable::create_buffer(
            &self.payer.pubkey(),
            &program_buffer_keypair.pubkey(),
            &program_upgrade_authority_keypair.pubkey(),
            program_buffer_rent,
            program_data.len(),
        )
        .unwrap();

        let chunk_size = 800;

        for (chunk, i) in program_data.chunks(chunk_size).zip(0..) {
            instructions.push(bpf_loader_upgradeable::write(
                &program_buffer_keypair.pubkey(),
                &program_upgrade_authority_keypair.pubkey(),
                (i * chunk_size) as u32,
                chunk.to_vec(),
            ));
        }

        let program_account_rent = self
            .rent
            .minimum_balance(UpgradeableLoaderState::program_len().unwrap());

        let deploy_instructions = bpf_loader_upgradeable::deploy_with_max_program_len(
            &self.payer.pubkey(),
            &program_address_keypair.pubkey(),
            &program_buffer_keypair.pubkey(),
            &program_upgrade_authority_keypair.pubkey(),
            program_account_rent,
            program_data.len(),
        )
        .unwrap();

        instructions.extend_from_slice(&deploy_instructions);

        self.process_transaction(
            &instructions[..],
            Some(&[
                &program_upgrade_authority_keypair,
                &program_address_keypair,
                &program_buffer_keypair,
            ]),
        )
        .await;

        GovernedProgramSetup {
            address: program_address_keypair.pubkey(),
            upgrade_authority: program_upgrade_authority_keypair,
            data_address: program_data_address,
        }
    }

    #[allow(dead_code)]
    pub async fn with_dummy_governed_program(&mut self) -> GovernedProgramSetup {
        GovernedProgramSetup {
            address: Pubkey::new_unique(),
            upgrade_authority: Keypair::new(),
            data_address: Pubkey::new_unique(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_program_governance(
        &mut self,
        governed_program: &GovernedProgramSetup,
    ) -> ProgramGovernanceSetup {
        let (governance_address, _) = Pubkey::find_program_address(
            &[PROGRAM_AUTHORITY_SEED, governed_program.address.as_ref()],
            &id(),
        );

        let governance_mint = Pubkey::new_unique();
        let council_mint = Option::None::<Pubkey>;

        let vote_threshold: u8 = 60;
        let min_instruction_hold_up_time: u64 = 10;
        let max_voting_time: u64 = 100;

        let create_governance_instruction = create_governance(
            &governance_address,
            &governed_program.address,
            &governed_program.data_address,
            &governed_program.upgrade_authority.pubkey(),
            &governance_mint,
            &self.payer.pubkey(),
            &council_mint,
            vote_threshold,
            min_instruction_hold_up_time,
            max_voting_time,
        )
        .unwrap();

        self.process_transaction(
            &[create_governance_instruction],
            Some(&[&governed_program.upgrade_authority]),
        )
        .await;

        ProgramGovernanceSetup {
            address: governance_address,
            governance_mint,
            council_mint,
            vote_threshold,
            min_instruction_hold_up_time,
            max_voting_time,
        }
    }

    #[allow(dead_code)]
    pub async fn get_program_governance_account(
        &mut self,
        governance_address: &Pubkey,
    ) -> ProgramGovernance {
        let governance_account_raw = self
            .banks_client
            .get_account(*governance_address)
            .await
            .unwrap()
            .unwrap();

        ProgramGovernance::unpack(&governance_account_raw.data).unwrap()
    }

    pub async fn get_account<T: BorshDeserialize>(&mut self, address: &Pubkey) -> T {
        let raw_account = self
            .banks_client
            .get_account(*address)
            .await
            .unwrap()
            .unwrap();

        T::try_from_slice(&raw_account.data).unwrap()
    }

    #[allow(dead_code)]
    pub async fn with_proposal(&mut self, governance: &ProgramGovernanceSetup) -> ProposalSetup {
        let description_link = "proposal description".to_string();
        let name = "proposal_name".to_string();

        //let proposal_count = 0;
        let proposal_key = Keypair::new();

        let create_proposal_instruction = create_proposal(
            description_link.clone(),
            name.clone(),
            &proposal_key.pubkey(),
            &governance.address,
            &self.payer.pubkey(),
        )
        .unwrap();

        self.process_transaction(&[create_proposal_instruction], Some(&[&proposal_key]))
            .await;

        ProposalSetup {
            address: proposal_key.pubkey(),
            description_link: description_link,
            name: name,
        }
    }

    #[allow(dead_code)]
    pub async fn with_root_governance(&mut self) -> RootGovernanceSetup {
        let name = "Root Governance".to_string();

        //let proposal_count = 0;
        let root_governance_key = get_root_governance_address(&name);

        let governance_token_mint_keypair = Keypair::new();
        let governance_token_mint_authority = Pubkey::new_unique();
        self.create_mint(
            &governance_token_mint_keypair,
            &governance_token_mint_authority,
        )
        .await;

        let governance_token_holding_keypair = Keypair::new();

        let council_mint_keypair = Keypair::new();
        let council_mint_authority = Pubkey::new_unique();
        self.create_mint(&council_mint_keypair, &council_mint_authority)
            .await;

        let council_token_holding_keypair = Keypair::new();

        let create_proposal_instruction = create_root_governance(
            name.clone(),
            &governance_token_mint_keypair.pubkey(),
            &governance_token_holding_keypair.pubkey(),
            &self.payer.pubkey(),
            Some(council_mint_keypair.pubkey()),
            Some(council_token_holding_keypair.pubkey()),
        )
        .unwrap();

        self.process_transaction(
            &[create_proposal_instruction],
            Some(&[
                &governance_token_holding_keypair,
                &council_token_holding_keypair,
            ]),
        )
        .await;

        RootGovernanceSetup {
            address: root_governance_key,
            name,
            governance_mint: governance_token_mint_keypair.pubkey(),
            council_mint: Some(council_mint_keypair.pubkey()),
        }
    }

    #[allow(dead_code)]
    pub async fn get_root_governnace_account(
        &mut self,
        root_governance_address: &Pubkey,
    ) -> RootGovernance {
        self.get_account::<RootGovernance>(root_governance_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_proposal_account(&mut self, proposal_address: &Pubkey) -> Proposal {
        self.get_account::<Proposal>(proposal_address).await
    }

    #[allow(dead_code)]
    async fn get_packed_account<T: Pack + IsInitialized>(&mut self, address: &Pubkey) -> T {
        let raw_account = self
            .banks_client
            .get_account(*address)
            .await
            .unwrap()
            .unwrap();

        T::unpack(&raw_account.data).unwrap()
    }

    pub async fn create_mint(&mut self, mint_keypair: &Keypair, mint_authority: &Pubkey) {
        let mint_rent = self.rent.minimum_balance(spl_token::state::Mint::LEN);

        let instructions = [
            system_instruction::create_account(
                &self.payer.pubkey(),
                &mint_keypair.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint_keypair.pubkey(),
                &mint_authority,
                None,
                0,
            )
            .unwrap(),
        ];

        self.process_transaction(&instructions, Some(&[&mint_keypair]))
            .await;
    }
}

fn get_governed_program_path(name: &str) -> PathBuf {
    let mut pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pathbuf.push("tests/program_test/programs");
    pathbuf.push(name);
    pathbuf.set_extension("_so");
    pathbuf
}

fn read_governed_program(name: &str) -> Vec<u8> {
    let path = get_governed_program_path(name);
    let mut file = File::open(&path).unwrap_or_else(|err| {
        panic!("Failed to open {}: {}", path.display(), err);
    });
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();

    elf
}
