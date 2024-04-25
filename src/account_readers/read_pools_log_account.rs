use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;

impl DvlReadable for PoolsLogAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    fn get_public_key<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.pools_log_address))

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
    use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_pools_log_account_by_index() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let _pool_log_0 = reader.get_account::<PoolsLogAccount>(DvlIndexParam { id: 0 })?;
        Ok(())
    }

    #[test]
    fn test_read_pools_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let workers_account = reader.get_account::<AllWorkersAccount>(())?;
        let pubkey = &workers_account.workers[0].pools_log_address;
        let _pool_log_0 = reader.get_account_by_public_key::<PoolsLogAccount>(pubkey)?;
        Ok(())
    }
}
