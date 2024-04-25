use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlClientParam, DvlReadable};
use crate::accounts::client::payoff_log::payoff_log_account::PayoffLogAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;

impl DvlReadable for PayoffLogAccount {
    type DvlReadParams<'a> = DvlClientParam<'a>;

    fn get_public_key<'a>(_: &DvlClient, params: &DvlClientParam) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        Ok(Box::from(params.client_account.payoff_log))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
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
    use crate::generate_pda::dvl_generate_pda;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_payoff_log_account_auto() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let client_pda = dvl_generate_pda(&reader.admin_public_key, &reader.main_seed, &reader.program_id);
        let client_account = reader.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_pda.key,
            signer_account_params: None,
        })?;

        let _payoff = reader.get_account::<PayoffLogAccount>(DvlClientParam {
            client_account: &client_account,
        })?;
        Ok(())
    }

    #[test]
    fn test_read_payoff_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let client_pda = dvl_generate_pda(&reader.admin_public_key, &reader.main_seed, &reader.program_id);
        let client_account = reader.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_pda.key,
            signer_account_params: None,
        })?;

        let _payoff = reader.get_account_by_public_key::<PayoffLogAccount>(&client_account.payoff_log)?;
        Ok(())
    }
}
