use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::root::root_account::RootAccount;

// impl DvlReadablePublicKey for WorkerAccount {}

impl DvlReadable for AllWorkersAccount {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let root = reader.read::<RootAccount>(None).unwrap();
        let pubkey = &root.workers_address;
        let mut rpc_data = reader.client.get_account(pubkey)?;
        let workers_account = AllWorkersAccount::from_account(
            pubkey,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;
        Ok(workers_account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::all_workers::all_workers_account::{ALL_WORKERS_ACCOUNT_TAG, ALL_WORKERS_ACCOUNT_VERSION, AllWorkersAccount};
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_root_account() {
        let reader = setup_account_reader();
        let all_worker_account = reader.read::<AllWorkersAccount>(None).unwrap();

        assert_eq!(all_worker_account.header.tag, ALL_WORKERS_ACCOUNT_TAG as u32);
        assert_eq!(all_worker_account.header.version, ALL_WORKERS_ACCOUNT_VERSION as u32);
    }
}

// use std::error::Error;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::account::ReadableAccount;
// use crate::account_readers::read_root_account::read_root_account;
// use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
// use crate::accounts::root::RootAccount;
// use crate::data_structure::ALL_WORKERS_ACCOUNT_SIZE;
//
// pub fn read_all_workers_account(client: &RpcClient) -> Result<AllWorkersAccount, Box<dyn Error>> {
//     let root : RootAccount = read_root_account(&client)?;
//     let key = root.workers_address;
//     let rpc_data = client.get_account(&key)?;
//     if rpc_data.data.len() != ALL_WORKERS_ACCOUNT_SIZE {
//         return Err("Incorrect all workers account size".into());
//     }
//     Ok(AllWorkersAccount::from(rpc_data.data()))
// }
//
// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//     use super::*;
//     use solana_client::rpc_client::RpcClient;
//     use solana_program::pubkey::Pubkey;
//     use crate::constants::ROOT_SEED;
//     use crate::env::{ADMIN_PUBLIC_KEY, PROGRAM_ID, RPC_URL};
//     use crate::generate_pda::generate_pda;
//
//     #[test]
//     fn test_read_all_workers_account() {
//         let client = RpcClient::new(String::from(RPC_URL));
//
//         let all_workers_account_result = read_all_workers_account(&client);
//
//         if let Err(e) = &all_workers_account_result {
//             println!("Error: {}", e);
//         }
//
//         assert!(all_workers_account_result.is_ok(), "The result should be ok but was Err");
//
//         let all_workers_account = all_workers_account_result.expect("Expected Ok value, got Err");
//         let root_key = Pubkey::from_str(&generate_pda(ADMIN_PUBLIC_KEY, ROOT_SEED.as_ref(), PROGRAM_ID)).unwrap();
//         assert_eq!(all_workers_account.root_address, root_key);
//     }
// }
