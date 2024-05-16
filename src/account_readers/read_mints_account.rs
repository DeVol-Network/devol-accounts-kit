use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::mints::mints_account::MintsAccount;
use crate::accounts::root::root_account::RootAccount;

#[async_trait]
impl DvlReadable for MintsAccount {
    async fn get_public_key<'a>(client: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let root = client.get_account::<RootAccount>(()).await?;
        Ok(Box::from(root.mints_address))
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[tokio::test]
    async fn test_read_mints_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _mints_account = client.get_account::<MintsAccount>(()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_mints_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let root_account = client.get_account::<RootAccount>(()).await?;
        let pubkey = &root_account.mints_address;
        let _mints_account = client.get_account_by_public_key::<MintsAccount>(pubkey).await?;
        Ok(())
    }
}
