use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlClientParam};
use crate::accounts::client::portfolio_account::portfolio_account::PortfolioAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::generate_pda::dvl_generate_pda;

#[async_trait]
impl DvlReadable for PortfolioAccount
{
    type DvlReadParams<'a> = DvlClientParam<'a>;

    async fn get_public_key<'a>(
        dvl_client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Pubkey>, Box<dyn Error>>
        where Self: Sized
        {
            let client_pda = dvl_generate_pda(
                &params.client_account.portfolio_address,
                &dvl_client.main_seed,
                &dvl_client.program_id
            );
            Ok(Box::from(client_pda.key))
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
    use crate::account_readers::dvl_readable::{DvlClientParam, DvlClientParams};
    use crate::accounts::client::client_account::client_account::ClientAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;
    use std::str::FromStr;
    use solana_program::pubkey::Pubkey;
    use crate::accounts::client::portfolio_account::portfolio_account::PortfolioAccount;
    use crate::constants::test_constants::{ADMIN_PUBLIC_KEY};

    #[tokio::test]
    async fn test_read_client_account() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let client_wallet_public_key = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();
        let client_account = client.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_wallet_public_key,
            signer_account_params: None,
        }).await?;
        let portfolio_account = client.get_account::<PortfolioAccount>(DvlClientParam {
            client_account: &client_account,
        }).await?;

        assert_eq!(portfolio_account.owner_address, client_wallet_public_key);

        Ok(())
    }
}
