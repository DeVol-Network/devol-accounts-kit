use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;

impl DvlReadablePublicKey for PoolsTraceAccount {}

impl DvlReadableIndexed for PoolsTraceAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let worker = workers_account.workers[index];
        let public_key = &worker.pools_trace_address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::worker::pools_trace::pools_trace_account::{POOLS_TRACE_ACCOUNT_TAG, POOLS_TRACE_ACCOUNT_VERSION, PoolsTraceAccount};
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_pools_trace_account() {
        let reader = setup_account_reader();
        // Test read by index
        let pool_trace_0 = reader.read_indexed::<PoolsTraceAccount>(0,None).unwrap();
        check_pools_trace_account(&pool_trace_0);
        // Test read by public key
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let pubkey = &workers_account.workers[0].pools_trace_address;
        let pool_trace_0 = reader.read_by_public_key::<PoolsTraceAccount>(pubkey,None).unwrap();
        check_pools_trace_account(&pool_trace_0);
    }

    fn check_pools_trace_account(mint_log_account: &PoolsTraceAccount){
        assert_eq!(mint_log_account.header.tag, POOLS_TRACE_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, POOLS_TRACE_ACCOUNT_VERSION);
    }
}
