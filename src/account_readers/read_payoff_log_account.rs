use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlClientParam, DvlReadable};
use crate::accounts::client::payoff_log::payoff_log_account::PayoffLogAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;

#[async_trait]
impl DvlReadable for PayoffLogAccount {
    type DvlReadParams<'a> = DvlClientParam<'a>;

    async fn get_public_key<'a>(
        _dvl_client: &DvlClient,
        params: &DvlClientParam
    ) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        Ok(Box::from(params.client_account.payoff_log))
    }

    async fn read<'a>(
        client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(client, params).await?;
        let mut rpc_data = client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &client.root_pda.key,
            &client.program_id,
            Some(params.client_account.id),
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_readers::dvl_readable::DvlClientParams;
    use crate::accounts::client::client_account::client_account::ClientAccount;
    use crate::accounts::client::payoff_log::payoff_log_account::PayoffLogAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;
    use std::str::FromStr;
    use crate::constants::test_constants::ADMIN_PUBLIC_KEY;

    #[tokio::test]
    async fn test_read_payoff_log_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let client_wallet_public_key = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();
        let client_account = client.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_wallet_public_key,
            signer_account_params: None,
        }).await?;

        let _payoff = client.get_account::<PayoffLogAccount>(DvlClientParam {
            client_account: &*client_account,
        }).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_payoff_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let client_wallet_public_key = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();
        let client_account = client.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_wallet_public_key,
            signer_account_params: None,
        }).await?;

        let _payoff = client.get_account_by_public_key::<PayoffLogAccount>(&client_account.payoff_log).await?;
        Ok(())
    }
}
