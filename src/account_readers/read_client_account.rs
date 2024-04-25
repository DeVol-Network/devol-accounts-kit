use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlClientParams};
use crate::accounts::client::client_account::client_account::ClientAccount;

impl DvlReadable for ClientAccount
{
    type DvlReadParams<'a> = DvlClientParams<'a>;

    fn get_public_key<'a>(_: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>>
        where Self: Sized {
        Ok(Box::from(*params.client_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            params.signer_account_params,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_readers::dvl_readable::DvlClientParams;
    use crate::accounts::client::client_account::client_account::ClientAccount;
    use crate::generate_pda::generate_pda;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_client_account() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let client_pda = generate_pda(&reader.admin_public_key, &reader.main_seed, &reader.program_id);
        let _client_account = reader.get_account::<ClientAccount>(DvlClientParams {
            client_address: &client_pda.key,
            signer_account_params: None,
        })?;
        Ok(())
    }
}

