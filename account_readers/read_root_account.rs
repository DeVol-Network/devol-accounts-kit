use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::{ReadableAccount};
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::account_transformer::{transform_account_data};
use crate::accounts::root::root_account::RootAccount;

impl DvlAccountReader {
    pub fn read_root_account(&self) -> Result<RootAccount, Box<dyn Error>> {
        let rpc_data = self.client.get_account(&self.root_pda.key)?;
        let root : RootAccount =  *transform_account_data(&rpc_data.data)?;
        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    use crate::data_structure::{ROOT_ACCOUNT_TAG, ROOT_ACCOUNT_VERSION};

    #[test]
    fn test_read_root_account() {
        let client = RpcClient::new(String::from(RPC_URL));
        let result = read_root_account(&client);

        if let Err(e) = &result {
            println!("Error: {}", e);
        }

        assert!(result.is_ok(), "The result should be ok but was Err");

        let root_account = result.expect("Expected Ok value, got Err");

        assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, ROOT_ACCOUNT_VERSION as u32);
    }
}
