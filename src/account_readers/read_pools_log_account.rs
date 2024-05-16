use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;

#[async_trait]
impl DvlReadable for PoolsLogAccount {
    async fn get_public_key<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = client.get_account::<AllWorkersAccount>(()).await?;
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.pools_log_address))

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

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
}
