use std::error::Error;
use async_trait::async_trait;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;

#[async_trait]
pub trait AsTransactionInstruction {
    type DvlTransactionInstructionParams;

    async fn as_transaction_instruction (
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams
    ) -> Result<Box<Instruction>, Box<dyn Error>>;
}