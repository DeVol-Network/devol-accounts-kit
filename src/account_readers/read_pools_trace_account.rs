use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;

impl DvlReadable for PoolsTraceAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    fn get_public_key<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.pools_trace_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
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
    use crate::accounts::worker::pools_trace::pools_trace_account::{POOLS_TRACE_ACCOUNT_TAG, POOLS_TRACE_ACCOUNT_VERSION, PoolsTraceAccount};
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_pools_trace_account() {
        let reader = setup_account_reader();
        // Test read by index
        let pool_trace_0 = reader.get_account::<PoolsTraceAccount>(DvlIndexParam {id: 0}).unwrap();
        check_pools_trace_account(&pool_trace_0);
        // Test read by public key
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let pubkey = &workers_account.workers[0].pools_trace_address;
        let pool_trace_0 = reader.get_account_by_public_key::<PoolsTraceAccount>(pubkey).unwrap();
        check_pools_trace_account(&pool_trace_0);
    }

    fn check_pools_trace_account(pools_trace_account: &PoolsTraceAccount){
        assert_eq!(pools_trace_account.header.tag, POOLS_TRACE_ACCOUNT_TAG as u32);
        assert_eq!(pools_trace_account.header.version, POOLS_TRACE_ACCOUNT_VERSION);
    }
}
