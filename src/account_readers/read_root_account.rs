use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::{ReadableAccount};
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::DvlReadable;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for RootAccount {
    fn read(reader: &DvlAccountReader) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let mut rpc_data = reader.client.get_account(&reader.root_pda.key)?;
        let root : RootAccount =  RootAccount::from_account(
            &reader.root_pda.key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            None).unwrap();
        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::root::root_account::{ROOT_ACCOUNT_TAG, ROOT_ACCOUNT_VERSION};
    use crate::constants::test_constants::{ADMIN_PUBLIC_KEY, INT_SEED, PROGRAM_ID, RPC_URL};

    #[test]
    fn test_read_root_account() {
        let client = RpcClient::new(String::from(RPC_URL));
        let reader = DvlAccountReader::new(client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
        let result = reader.read::<RootAccount>();

        if let Err(e) = &result {
            println!("Error: {}", e);
        }

        assert!(result.is_ok(), "The result should be ok but was Err");

        let root_account = result.expect("Expected Ok value, got Err");

        assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, ROOT_ACCOUNT_VERSION);
    }
}
