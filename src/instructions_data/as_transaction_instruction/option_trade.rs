use std::error::Error;
use solana_program::instruction::Instruction;
use solana_sdk::signature::Keypair;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::option_trade::InstructionOptionTrade;

impl AsTransactionInstruction for InstructionOptionTrade{
    type DvlTransactionInstructionParams = ();

    fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Keypair,
        transaction_params: Self::DvlTransactionInstructionParams
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        todo!()
    }
}