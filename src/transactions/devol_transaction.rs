use std::error::Error;
use solana_sdk::transaction::Transaction;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions::devol_instruction_data::DevolInstructionData;

// pub trait DevolTransaction {
//     type DvlTransactionParams;
//
//     fn as_transaction (
//         &self,
//         dvl_client: &DvlClient,
//         transaction_params: Self::DvlTransactionParams
//     ) -> Result<Box<Transaction>, Box<dyn Error>>
//         where Self: DevolInstructionData;
// }