use std::error::Error;
use solana_program::instruction::Instruction;
use solana_sdk::signature::Keypair;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions::devol_instruction_data::DevolInstructionData;

pub trait DevolTransaction {
    type DvlTransactionParams;

    fn as_transaction_instruction (
        &self,
        client: &DvlClient,
        signer: &Keypair,
        transaction_params: Self::DvlTransactionParams
    ) -> Result<Box<Instruction>, Box<dyn Error>>
        where Self: DevolInstructionData;
}