use std::error::Error;
use solana_sdk::transaction::Transaction;
use crate::account_readers::dvl_readable::DvlIndexParam;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::start_next_pool::InstructionStartNextPool;
// use crate::transactions::devol_transaction::DevolTransaction;

// pub struct StartNextPoolTransactionParams {
//     pub worker_id: usize,
// }
//
// impl DevolTransaction for InstructionStartNextPool {
//     type DvlTransactionParams = StartNextPoolTransactionParams;
//
//     fn as_transaction (
//         &self,
//         dvl_client: &DvlClient,
//         transaction_params: Self::DvlTransactionParams
//     ) -> Result<Box<Transaction>, Box<dyn Error>>
//         where Self: DevolInstructionData {
//         let data = self.as_vec_le();
//         let worker_acc = dvl_client.get_account::<WorkerAccount>(DvlIndexParam { id: 0 })?;
//         let root_acc_key = dvl_client.root_pda.key;
//
//     }
// }