use std::str::FromStr;

use solana_program::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
};
use solana_program_test::{processor, ProgramTest, ProgramTestContext};

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_governance_chat::{
    instruction::post_message, processor::process_instruction, state::Message,
};
use spl_governance_test_sdk::tools::map_transaction_error;

use self::cookies::MessageCookie;

pub mod cookies;

pub struct GovernanceChatProgramTest {
    pub context: ProgramTestContext,
    pub rent: Rent,
    pub next_realm_id: u8,
    pub program_id: Pubkey,
}

impl GovernanceChatProgramTest {
    pub async fn start_new() -> Self {
        let program_id = Pubkey::from_str("GovernanceChat11111111111111111111111111111").unwrap();

        let mut program_test = ProgramTest::new(
            "spl_governance_chat",
            program_id,
            processor!(process_instruction),
        );

        let governance_program_id =
            Pubkey::from_str("Governance111111111111111111111111111111111").unwrap();

        program_test.add_program(
            "spl_governance",
            governance_program_id,
            processor!(spl_governance::processor::process_instruction),
        );

        let mut context = program_test.start_with_context().await;
        let rent = context.banks_client.get_rent().await.unwrap();

        Self {
            context,
            rent,
            next_realm_id: 0,
            program_id,
        }
    }

    pub async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: Option<&[&Keypair]>,
    ) -> Result<(), ProgramError> {
        let mut transaction =
            Transaction::new_with_payer(instructions, Some(&self.context.payer.pubkey()));

        let mut all_signers = vec![&self.context.payer];

        if let Some(signers) = signers {
            all_signers.extend_from_slice(signers);
        }

        let recent_blockhash = self
            .context
            .banks_client
            .get_recent_blockhash()
            .await
            .unwrap();

        transaction.sign(&all_signers, recent_blockhash);

        self.context
            .banks_client
            .process_transaction(transaction)
            .await
            .map_err(map_transaction_error)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn with_message(&mut self) -> MessageCookie {
        let proposal = Pubkey::new_unique();

        let post_message_ix = post_message(
            &self.program_id,
            &self.context.payer.pubkey(),
            &self.context.payer.pubkey(),
        );

        let message = Message {
            proposal: Pubkey::new_unique(),
            author: Pubkey::new_unique(),
            post_at: 10,
            parent: None,
            body: "post ".to_string(),
        };

        self.process_transaction(&[post_message_ix], None)
            .await
            .unwrap();

        MessageCookie {
            address: Pubkey::new_unique(),
            account: message,
        }
    }
}
