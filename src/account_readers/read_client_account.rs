use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, SignableAccountParams};
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for ClientAccount {
    type AdditionalCheckParams =  SignableAccountParams;
    fn read(reader: &DvlAccountReader, params: Self::AdditionalCheckParams
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = params.client_address;
        // let account =  Self::read_by_public_key(reader, public_key, Some(_params))?;
        let mut rpc_data = reader.client.get_account(&*public_key)?;
        let account =  Self::from_account(
            &*public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            &*params.signer,
            params.devol_sign
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::client::client_account::client_account::{CLIENT_ACCOUNT_TAG, CLIENT_ACCOUNT_VERSION};
    use super::*;
    use crate::tests::tests::setup_account_reader;
    #[test]
    fn test_read_client_account() {
        println!("1");
        let reader = setup_account_reader();
        println!("2");
        // Test auto read
        let client_account = reader.read::<ClientAccount>(SignableAccountParams{
            client_address: Box::new(reader.admin_public_key),
            signer: Box::new(reader.admin_public_key),
            devol_sign: false,
        }).unwrap();
        println!("3");
        check_client_account(&client_account);
        // Test read by public key
        let public_key = reader.admin_public_key;
        let client_account = reader.read_by_public_key::<ClientAccount>(&public_key).unwrap();
        check_client_account(&client_account);
    }

    fn check_client_account(client_account: &ClientAccount){
        assert_eq!(client_account.header.tag, CLIENT_ACCOUNT_TAG as u32);
        assert_eq!(client_account.header.version, CLIENT_ACCOUNT_VERSION as u32);
    }
}
