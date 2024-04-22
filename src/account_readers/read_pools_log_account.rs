use std::error::Error;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, IndexedAccountParams};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;

impl DvlReadable for PoolsLogAccount {
    type AdditionalCheckParams<'a> = IndexedAccountParams;

    fn read<'a>(reader: &DvlClient, params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id as usize];
        let public_key = &worker.pools_log_address;
        // let account =  Self::read_by_public_key(reader, public_key, Some(params))?;
        let mut rpc_data = reader.client.get_account(public_key)?;
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
    use crate::accounts::worker::pools_log::pools_log_account::{POOLS_LOG_ACCOUNT_TAG, POOLS_LOG_ACCOUNT_VERSION, PoolsLogAccount};
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_pools_log_account() {
        let reader = setup_account_reader();
        // Test read by index
        let pool_log_0 = reader.read::<PoolsLogAccount>(IndexedAccountParams {id: 0}).unwrap();
        check_pools_log_account(&pool_log_0);
        // Test read by public key
        let workers_account = reader.read::<AllWorkersAccount>(()).unwrap();
        let pubkey = &workers_account.workers[0].pools_log_address;
        let pool_log_0 = reader.read_by_public_key::<PoolsLogAccount>(pubkey).unwrap();
        check_pools_log_account(&pool_log_0);
    }

    fn check_pools_log_account(pools_log_account: &PoolsLogAccount){
        assert_eq!(pools_log_account.header.tag, POOLS_LOG_ACCOUNT_TAG as u32);
        assert_eq!(pools_log_account.header.version, POOLS_LOG_ACCOUNT_VERSION);
    }
}
