use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::{ReadableAccount};
use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_SIZE, OraclesAccount};
use crate::accounts::root::RootAccount;
use crate::constants::*;
use crate::data_structure::ROOT_ACCOUNT_SIZE;
use crate::env::*;
use crate::generate_pda::generate_pda;


pub fn read_oracles_account(client: &RpcClient) -> Result<OraclesAccount, Box<dyn Error>> {
    let key = Pubkey::from_str(&generate_pda(ADMIN_PUBLIC_KEY, ORACLE_SEED.as_ref(), PROGRAM_ID))?;
    let rpc_data = client.get_account(&key)?;
    if rpc_data.data.len() != ORACLES_ACCOUNT_SIZE {
        return Err("Incorrect oracles account size".into());
    }
    Ok(OraclesAccount::from(rpc_data.data()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_TAG, ORACLES_ACCOUNT_VERSION};
    use crate::data_structure::{ROOT_ACCOUNT_TAG, ROOT_ACCOUNT_VERSION};

    #[test]
    fn test_read_oracles_account() {
        let client = RpcClient::new(String::from(RPC_URL));
        let result = read_oracles_account(&client);

        if let Err(e) = &result {
            println!("Error: {}", e);
        }

        assert!(result.is_ok(), "The result should be ok but was Err");

        let root_account = result.expect("Expected Ok value, got Err");

        assert_eq!(root_account.header.tag, ORACLES_ACCOUNT_TAG as u32);
        assert_eq!(root_account.header.version, ORACLES_ACCOUNT_VERSION as u32);
    }
}
