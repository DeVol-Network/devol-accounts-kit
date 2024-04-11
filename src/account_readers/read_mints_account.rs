use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mints_account::MintsAccount;
use crate::accounts::root::root_account::RootAccount;


impl DvlReadable for MintsAccount {
    type AdditionalCheckParams<'a> = ();

    fn read<'a>(reader: &DvlAccountReader, _params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let root = reader.read::<RootAccount>(()).unwrap();
        let public_key = &root.mints_address;
        // let account =  Self::read_by_public_key(reader, public_key, Some(_params))?;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account = Self::from_account_basic(
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
        let mints_account = reader.read::<MintsAccount>(()).unwrap();
        check_mints_account(&mints_account);
        // Test read by public key
        let root_account = reader.read::<RootAccount>(()).unwrap();
        let pubkey = &root_account.mints_address;
        let mints_account = reader.read_by_public_key::<MintsAccount>(pubkey).unwrap();
        check_mints_account(&mints_account);
    }


    fn check_mints_account(root_account: &MintsAccount){
        assert_eq!(root_account.header.tag, MINTS_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, MINTS_ACCOUNT_VERSION);
    }
}