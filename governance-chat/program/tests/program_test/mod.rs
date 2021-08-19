use std::{borrow::Borrow, rc::Rc, str::FromStr};

use solana_program::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
};
use solana_program_test::{processor, ProgramTest, ProgramTestContext};

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_governance_chat::{
    instruction::post_message, processor::process_instruction, state::Message,
};
use spl_governance_test_sdk::{
    tools::{clone_keypair, map_transaction_error},
    GovernanceProgramTest, TestBenchProgram,
};

use self::cookies::MessageCookie;

pub mod cookies;

pub struct GovernanceChatProgramTest {
    pub governance: GovernanceProgramTest,
    pub program_id: Pubkey,
    pub payer: Keypair,
}

impl GovernanceChatProgramTest {
    pub async fn start_new() -> Self {
        let program_id = Pubkey::from_str("GovernanceChat11111111111111111111111111111").unwrap();
        let program = TestBenchProgram {
            program_name: "spl_governance_chat",
            program_id: program_id,
            process_instruction: processor!(process_instruction),
        };

        let bench = GovernanceProgramTest::start_with_programs(&[program]).await;
        let payer = clone_keypair(&bench.context.payer);

        Self {
            governance: bench,
            program_id,
            payer,
        }
    }

    pub fn bench(&mut self) -> &mut GovernanceProgramTest {
        &mut self.governance
    }

    #[allow(dead_code)]
    pub async fn with_message(&mut self) -> MessageCookie {
        let proposal = Pubkey::new_unique();

        let post_message_ix =
            post_message(&self.program_id, &self.payer.pubkey(), &self.payer.pubkey());

        let message = Message {
            proposal: Pubkey::new_unique(),
            author: Pubkey::new_unique(),
            post_at: 10,
            parent: None,
            body: "post ".to_string(),
        };

        self.bench()
            .process_transaction(&[post_message_ix], None)
            .await
            .unwrap();

        MessageCookie {
            address: Pubkey::new_unique(),
            account: message,
        }
    }
}
