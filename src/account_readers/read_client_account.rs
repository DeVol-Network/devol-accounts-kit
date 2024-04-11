use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, SignableAccountParams};
use crate::accounts::client::client_account::client_account::ClientAccount;

impl DvlReadable for ClientAccount
{
    type AdditionalCheckParams<'a> = SignableAccountParams<'a>;
    fn read<'a>(reader: &DvlAccountReader, params: Self::AdditionalCheckParams<'a>,
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*params.client_address;
        let mut rpc_data = reader.client.get_account(public_key)?;
        if let Some(signer_params) = params.signer_account {
            let account = Self::from_account(
                public_key,
                &mut rpc_data,
                &reader.root_pda.key,
                &reader.program_id,
                &*signer_params.signer,
                signer_params.devol_sign,
            )?;
            Ok(account)
        } else {
            let account = Self::from_account(
                public_key,
                &mut rpc_data,
                &reader.root_pda.key,
                &reader.program_id,
                &Pubkey::default(),
                true,
            )?;
            Ok(account)
        }
        // Ok(Box::from(ClientAccount::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::client::client_account::client_account::{CLIENT_ACCOUNT_TAG, CLIENT_ACCOUNT_VERSION};
    use crate::generate_pda::generate_pda;
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_client_account() {
        let reader = setup_account_reader();
        let client_pda = generate_pda(&reader.admin_public_key, &reader.main_seed, &reader.program_id);
        // Test auto read
        let client_account = reader.read::<ClientAccount>(SignableAccountParams {
            client_address: &client_pda.key,
            signer_account: None,
        }).unwrap();
        check_client_account(&client_account);
        // Test read by public key
        // let public_key = reader.admin_public_key;
        // let client_account = reader.read_by_public_key::<ClientAccount>(&public_key).unwrap();
        // check_client_account(&client_account);
    }

    fn check_client_account(client_account: &ClientAccount) {
        assert_eq!(client_account.header.tag, CLIENT_ACCOUNT_TAG as u32);
        assert_eq!(client_account.header.version, CLIENT_ACCOUNT_VERSION as u32);
    }
}