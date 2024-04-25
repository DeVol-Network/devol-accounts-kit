use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::worker_account::WorkerAccount;

impl DvlReadable for WorkerAccount {
    type DvlReadParams<'a> = DvlIndexParam;
    fn get_public_key<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            Some(params.id),
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::worker::worker_account::{WORKER_ACCOUNT_TAG, WORKER_ACCOUNT_VERSION};
    use crate::tests::tests::setup_devol_client;

    #[test]
    fn test_read_worker_account() {
        let reader = setup_devol_client();
        // Test read by index
        let worker_0 = reader.get_account::<WorkerAccount>(DvlIndexParam {id: 0}).unwrap();
        check_worker_account(&worker_0);
        // Test read by public key
        let mints_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let pubkey = &mints_account.workers[0].address;
        let worker_0 = reader.get_account_by_public_key::<WorkerAccount>(pubkey).unwrap();
        check_worker_account(&worker_0);
    }

    fn check_worker_account(mint_log_account: &WorkerAccount){
        assert_eq!(mint_log_account.header.tag, WORKER_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, WORKER_ACCOUNT_VERSION);
    }
}
