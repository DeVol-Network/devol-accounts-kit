use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadablePublicKey};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadablePublicKey for RootAccount {}

impl DvlReadable for RootAccount {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let mut rpc_data = reader.client.get_account(&reader.root_pda.key)?;
        let root : RootAccount =  RootAccount::from_account(
            &reader.root_pda.key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;
        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::root::root_account::{ROOT_ACCOUNT_TAG, ROOT_ACCOUNT_VERSION};
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_root_account() {
        let reader = setup_account_reader();
        let root = reader.read::<RootAccount>(None).unwrap();
        let root2 = reader.read_by_public_key::<RootAccount>(&reader.root_pda.key, None).unwrap();
        // let root = reader.read_by_public_key::<RootAccount>(&reader.root_pda.key, None).unwrap();
        // let root = reader.read_indexed::<RootAccount>(0).unwrap();

        assert_eq!(root.header.tag, ROOT_ACCOUNT_TAG as u32);
        assert_eq!(root.header.version, ROOT_ACCOUNT_VERSION);
    }
}
