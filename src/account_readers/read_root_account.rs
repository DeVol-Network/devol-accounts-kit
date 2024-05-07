use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::root::root_account::RootAccount;

#[async_trait]
impl DvlReadable for RootAccount {
    type DvlReadParams<'a> = ();

    async fn get_public_key<'a>(client: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        Ok(Box::from(client.root_pda.key))
    }

    async fn read<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(client, params).await?;
        let mut rpc_data = client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &client.root_pda.key,
            &client.program_id,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use solana_program::pubkey::Pubkey;
    use crate::accounts::root::root_account::RootAccount;
    use crate::constants::test_constants::ROOT_ADDRESS;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[tokio::test]
    async fn test_read_root_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _root_account = client.get_account::<RootAccount>(()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_root_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let public_key = Pubkey::from_str(ROOT_ADDRESS)?;
        let client = setup_devol_client();
        let _root_account = client.get_account_by_public_key::<RootAccount>(&public_key).await?;
        Ok(())
    }
}
