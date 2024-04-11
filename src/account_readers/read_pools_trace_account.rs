use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, IndexedAccountParams};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;

impl DvlReadable for PoolsTraceAccount {
    type AdditionalCheckParams<'a> = IndexedAccountParams;

    fn read<'a>(reader: &DvlAccountReader, params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id];
        let public_key = &worker.pools_trace_address;
        // let account =  Self::read_by_public_key(reader, public_key)?;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            params.id,
        )?;
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
        let pool_trace_0 = reader.read::<PoolsTraceAccount>(IndexedAccountParams {id: 0}).unwrap();
        check_pools_trace_account(&pool_trace_0);
        // Test read by public key
        let workers_account = reader.read::<AllWorkersAccount>(()).unwrap();
        let pubkey = &workers_account.workers[0].pools_trace_address;
        let pool_trace_0 = reader.read_by_public_key::<PoolsTraceAccount>(pubkey).unwrap();
        check_pools_trace_account(&pool_trace_0);
    }

    fn check_pools_trace_account(pools_trace_account: &PoolsTraceAccount){
        assert_eq!(pools_trace_account.header.tag, POOLS_TRACE_ACCOUNT_TAG as u32);
        assert_eq!(pools_trace_account.header.version, POOLS_TRACE_ACCOUNT_VERSION);
    }
}
