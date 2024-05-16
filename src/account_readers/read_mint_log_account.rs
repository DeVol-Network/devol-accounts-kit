use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
use crate::accounts::mints::mints_account::MintsAccount;

#[async_trait]
impl DvlReadable for MintLogAccount {
    async fn get_public_key<'a>(clinet: &DvlClient, params: &DvlIndexParam) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let mints_account = clinet.get_account::<MintsAccount>(()).await?;
        Ok(Box::from(mints_account.data[params.id as usize].log_address))
    }
}

#[cfg(test)]
mod tests {
    use crate::account_readers::dvl_readable::DvlIndexParam;
    use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[tokio::test]
    async fn test_read_mint_log_account_by_index() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _mint_log_0 = client.get_account::<MintLogAccount>(DvlIndexParam { id: 0 }).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_mint_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let mints_account = client.get_account::<MintsAccount>(()).await?;
        let pubkey = &mints_account.data[0].log_address;
        let _mint_log_0 = client.get_account_by_public_key::<MintLogAccount>(pubkey).await?;
        Ok(())
    }
}