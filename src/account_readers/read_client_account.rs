use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadableClient, DvlReadablePublicKey};
use crate::accounts::client::client_account::client_account::ClientAccount;

impl DvlReadableClient for ClientAccount {
    fn read(
        reader: &DvlAccountReader,
        account_address: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let account = Self::read(reader, account_address, signer, devol_sign)?;
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
        // println!("1");
        // let reader = setup_account_reader();
        // println!("2");
        // Test auto read
        // let client_account = reader.read_client::<ClientAccount>(&reader.admin_public_key, &reader.admin_public_key, ).unwrap();
        // println!("3");
        // check_client_account(&client_account);
        // // Test read by public key
        // let public_key = reader.admin_public_key;
        // let client_account = reader.read_by_public_key::<ClientAccount>(&public_key ,None).unwrap();
        // check_client_account(&client_account);
    }

    fn check_client_account(client_account: &ClientAccount){
        assert_eq!(client_account.header.tag, CLIENT_ACCOUNT_TAG as u32);
        assert_eq!(client_account.header.version, CLIENT_ACCOUNT_VERSION as u32);
    }
}
