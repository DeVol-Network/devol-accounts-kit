use std::error::Error;
use solana_program::instruction::Instruction;
use solana_sdk::signature::Keypair;
use crate::dvl_client::dvl_client::DvlClient;

pub trait AsTransactionInstruction {
    type DvlTransactionInstructionParams;

    fn as_transaction_instruction (
        &self,
        client: &DvlClient,
        signer: &Keypair,
        transaction_params: Self::DvlTransactionInstructionParams
    ) -> Result<Box<Instruction>, Box<dyn Error>>;
}