use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::mints::mints_account::MintsAccount;
use crate::accounts::root::root_account::RootAccount;


impl DvlReadable for MintsAccount {
    type DvlReadParams<'a> = ();

    fn get_public_key<'a>(reader: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let root = reader.get_account::<RootAccount>(()).unwrap();
        Ok(Box::from(root.mints_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::mints::mints_account::*;
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_mints_account() {
        let reader = setup_account_reader();
        // Test auto read
        let mints_account = reader.get_account::<MintsAccount>(()).unwrap();
        check_mints_account(&mints_account);
        // Test read by public key
        let root_account = reader.get_account::<RootAccount>(()).unwrap();
        let pubkey = &root_account.mints_address;
        let mints_account = reader.get_account_by_public_key::<MintsAccount>(pubkey).unwrap();
        check_mints_account(&mints_account);
    }


    fn check_mints_account(root_account: &MintsAccount){
        assert_eq!(root_account.header.tag, MINTS_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, MINTS_ACCOUNT_VERSION);
    }
}