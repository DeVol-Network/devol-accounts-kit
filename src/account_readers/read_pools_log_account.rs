use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;

impl DvlReadablePublicKey for PoolsLogAccount {}

impl DvlReadableIndexed for PoolsLogAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let worker = workers_account.workers[index];
        let public_key = &worker.pools_log_address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
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
        let pool_log_0 = reader.read_indexed::<PoolsLogAccount>(0,None).unwrap();
        check_pools_log_account(&pool_log_0);
        // Test read by public key
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let pubkey = &workers_account.workers[0].pools_log_address;
        let pool_log_0 = reader.read_by_public_key::<PoolsLogAccount>(pubkey,None).unwrap();
        check_pools_log_account(&pool_log_0);
    }

    fn check_pools_log_account(pools_log_account: &PoolsLogAccount){
        assert_eq!(pools_log_account.header.tag, POOLS_LOG_ACCOUNT_TAG as u32);
        assert_eq!(pools_log_account.header.version, POOLS_LOG_ACCOUNT_VERSION);
    }
}
