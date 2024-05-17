use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::tasks_log::tasks_log_account::TasksLogAccount;

#[async_trait]
impl DvlReadable for TasksLogAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    async fn get_public_key<'a>(
        dvl_client: &DvlClient,
        params: &DvlIndexParam
    ) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = dvl_client.get_account::<AllWorkersAccount>(()).await?;
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.tasks_log_address))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::worker::tasks_log::tasks_log_account::TasksLogAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[tokio::test]
    async fn test_read_tasks_log_account_by_index() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _task_log_0 = client.get_account::<TasksLogAccount>(DvlIndexParam { id: 0 }).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_tasks_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let workers_account = client.get_account::<AllWorkersAccount>(()).await?;
        let pubkey = &workers_account.workers[0].tasks_log_address;
        let _task_log_0 = client.get_account_by_public_key::<TasksLogAccount>(pubkey).await?;
        Ok(())
    }
}
