use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlClientParams, DvlParametrable};
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::generate_pda::dvl_generate_pda;

impl DvlParametrable for ClientAccount { type DvlReadParams<'a> = DvlClientParams<'a>; }

#[async_trait]
impl DvlReadable for ClientAccount
{
    async fn get_public_key<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>>
        where Self: Sized
        {
            let client_pda = dvl_generate_pda(params.client_address, &client.main_seed, &client.program_id);
            Ok(Box::from(client_pda.key))
        }
}

#[cfg(test)]
mod tests {
    use crate::account_readers::dvl_readable::{DvlClientParams};
    use crate::accounts::client::client_account::client_account::ClientAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;
    use std::str::FromStr;
    use solana_program::pubkey::Pubkey;
    use crate::constants::test_constants::ADMIN_PUBLIC_KEY;

    #[tokio::test]
    async fn test_read_client_account() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let client_wallet_public_key = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();
        let _client_account = client.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_wallet_public_key,
            signer_account_params: None,
        }).await?;
        Ok(())
    }
}
