use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlClientParam, DvlParametrable, DvlReadable};
use crate::accounts::client::payoff_log::payoff_log_account::PayoffLogAccount;

impl DvlParametrable for PayoffLogAccount {     type DvlReadParams<'a> = DvlClientParam<'a>;
}

#[async_trait]
impl DvlReadable for PayoffLogAccount {
    async fn get_public_key<'a>(_: &DvlClient, params: &DvlClientParam) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        Ok(Box::from(params.client_account.payoff_log))
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
