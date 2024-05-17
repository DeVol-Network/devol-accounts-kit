use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::root::root_account::RootAccount;

#[async_trait]
impl DvlReadable for AllWorkersAccount {
    type DvlReadParams<'a> = ();

    async fn get_public_key<'a>(
        dvl_client: &DvlClient,
        _params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Pubkey>, Box<dyn Error>>
        where Self: Sized {
        let root = dvl_client.get_account::<RootAccount>(()).await?;
        Ok(Box::from(root.workers_address))
    }

    async fn read<'a>(
        dvl_client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(dvl_client, params).await?;
        let mut rpc_data = dvl_client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &dvl_client.root_pda.key,
            &dvl_client.program_id,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[tokio::test]
    async fn test_read_all_workers_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _all_workers_account = client.get_account::<AllWorkersAccount>(()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_all_workers_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let root_account = client.get_account::<RootAccount>(()).await?;
        let pubkey = &root_account.workers_address;
        let _all_workers_account = client.get_account_by_public_key::<AllWorkersAccount>(pubkey).await?;
        Ok(())
    }
}
