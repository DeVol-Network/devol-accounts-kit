use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for RootAccount {
    type AccountParam = ();

    fn read(reader: &DvlAccountReader, _params: Self::AccountParam) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        // let account =  Self::read_by_public_key(reader, &reader.root_pda.key)?;
        let public_key = &reader.root_pda.key;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account(
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
    use std::str::FromStr;
    use solana_program::pubkey::Pubkey;
    use super::*;
    use crate::accounts::root::root_account::{ROOT_ACCOUNT_TAG, ROOT_ACCOUNT_VERSION};
    use crate::constants::test_constants::ROOT_ADDRESS;
    use crate::tests::tests::setup_account_reader;
    #[test]
    fn test_read_root_account() {
        let reader = setup_account_reader();
        // Test auto read
        let root_account = reader.read::<RootAccount>(()).unwrap();
        check_root_account(&root_account);
        // Test read by public key
        // let public_key = Pubkey::from_str(ROOT_ADDRESS).unwrap();
        // let root_account = reader.read_by_public_key::<RootAccount>(&public_key ,None).unwrap();
        // check_root_account(&root_account);
    }

    fn check_root_account(root_account: &RootAccount){
        assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, ROOT_ACCOUNT_VERSION);
    }
}
