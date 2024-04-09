use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::worker::worker_account::WorkerAccount;

// impl DvlReadablePublicKey for WorkerAccount {}

impl DvlReadableIndexed for WorkerAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let worker = workers_account.workers[index];
        let public_key = &worker.address;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let worker_account =  WorkerAccount::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;
        Ok(worker_account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::worker::worker_account::{WORKER_ACCOUNT_TAG, WORKER_ACCOUNT_VERSION};
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_root_account() {
        let reader = setup_account_reader();
        let worker_0 = reader.read_indexed::<WorkerAccount>(0, None).unwrap();

        assert_eq!(worker_0.header.tag, WORKER_ACCOUNT_TAG as u32);
        assert_eq!(worker_0.header.version, WORKER_ACCOUNT_VERSION);
    }
}


// use std::error::Error;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::account::ReadableAccount;
// use crate::account_readers::read_all_workers_account::read_all_workers_account;
// use crate::accounts::worker::worker_account::WorkerAccount;
// use crate::data_structure::{WORKER_ACCOUNT_SIZE};
//
// pub fn read_worker_account(client: &RpcClient, id: usize) -> Result<WorkerAccount, Box<dyn Error>> {
//     let all_workers = read_all_workers_account(&client)?;
//     let key = all_workers.workers[id].address;
//     let rpc_data = client.get_account(&key)?;
//     if rpc_data.data.len() != WORKER_ACCOUNT_SIZE {
//         return Err("Incorrect worker account size".into());
//     }
//     Ok(WorkerAccount::from(rpc_data.data()))
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use solana_client::rpc_client::RpcClient;
//     use crate::data_structure::{WORKER_ACCOUNT_TAG, WORKER_ACCOUNT_VERSION};
//     use crate::env::RPC_URL;
//
//     #[test]
//     fn test_read_worker_account() {
//         let client = RpcClient::new(String::from(RPC_URL));
//
//         let worker_id = 0;
//
//         let worker_account_result = read_worker_account(&client, worker_id);
//
//         if let Err(e) = &worker_account_result {
//             println!("Error: {}", e);
//         }
//
//         assert!(worker_account_result.is_ok(), "The result should be ok but was Err");
//
//         let worker_account = worker_account_result.expect("Expected Ok value, got Err");
//
//         assert_eq!(worker_account.header.tag, WORKER_ACCOUNT_TAG as u32, "Worker account tag mismatch");
//         assert_eq!(worker_account.header.version, WORKER_ACCOUNT_VERSION as u32, "Worker account version mismatch");
//     }
// }
