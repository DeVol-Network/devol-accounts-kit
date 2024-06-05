use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;

#[async_trait]
impl DvlReadable for PoolsLogAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    async fn get_public_key<'a>(
        dvl_client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = dvl_client.get_account::<AllWorkersAccount>(()).await?;
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.pools_log_address))
    }

    async fn read<'a>(
        dvl_client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(dvl_client, params).await?;
        let mut rpc_data = dvl_client.rpc_client.get_account(public_key).await?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &dvl_client.root_pda.key,
            &dvl_client.program_id,
            Some(params.id),
        )?;
        Ok(account)
    }
}

// #[cfg(not(feature = "pools_log_migration"))]
#[cfg(test)]
mod tests {
    use std::{env, fs};
    use super::*;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;

    #[tokio::test]
    async fn test_read_pools_log_account_by_index() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _pool_log_0 = client.get_account::<PoolsLogAccount>(DvlIndexParam { id: 0 }).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_pools_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let workers_account = client.get_account::<AllWorkersAccount>(()).await?;
        let pubkey = &workers_account.workers[0].pools_log_address;
        let _pool_log_0 = client.get_account_by_public_key::<PoolsLogAccount>(pubkey).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_account() -> Result<(), Box<dyn Error>> {
        let dvl_client = setup_devol_client();
        let workers_account = dvl_client.get_account::<AllWorkersAccount>(()).await?;
        let current_dir = env::current_dir().unwrap();
        let pools_backup_dir = format!("{}/backup/new_impact_smart_contract/pools_log", current_dir.display());
        fs::create_dir_all(&pools_backup_dir)?;
        for i in 0..=7 {
            let pubkey = &workers_account.workers[0].pools_log_address;
            let _pool_log = dvl_client.rpc_client.get_account(pubkey).await?;
            let file_name = format!("{}/pool_log_{}.bin", pools_backup_dir, i);
            let mut file = File::create(file_name).expect("Unable to create file");
            file.write_all(&_pool_log.data).expect("Unable to write data");
        }
        Ok(())
    }
}
