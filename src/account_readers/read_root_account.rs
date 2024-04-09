use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadablePublicKey};
use crate::accounts::root::root_account::RootAccount;

impl DvlReadablePublicKey for RootAccount {}

impl DvlReadable for RootAccount {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let account =  Self::read_by_public_key(reader, &reader.root_pda.key, id)?;
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
        let root_account = reader.read::<RootAccount>(None).unwrap();
        check_root_account(&root_account);
        // Test read by public key
        let public_key = Pubkey::from_str(ROOT_ADDRESS).unwrap();
        let root_account = reader.read_by_public_key::<RootAccount>(&public_key ,None).unwrap();
        check_root_account(&root_account);
    }

    fn check_root_account(root_account: &RootAccount){
        assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, ROOT_ACCOUNT_VERSION);
    }
}
