use std::error::Error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use crate::account_readers::read_root_account::read_root_account;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::root::RootAccount;
use crate::data_structure::{ INSTR_ACCOUNT_SIZE};

pub fn read_instruments_account(client: &RpcClient) -> Result<InstrumentsAccount, Box<dyn Error>> {
    let root : RootAccount = read_root_account(&client)?;
    let key = root.instruments_address;
    let rpc_data = client.get_account(&key)?;
    if rpc_data.data.len() != INSTR_ACCOUNT_SIZE {
        return Err("Incorrect instruments account size".into());
    }
    Ok(InstrumentsAccount::from(rpc_data.data()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    use crate::data_structure::{INSTR_ACCOUNT_TAG, INSTR_ACCOUNT_VERSION};
    use crate::env::RPC_URL;

    #[test]
    fn test_read_instruments_account() {
        let client = RpcClient::new(String::from(RPC_URL));

        let instruments_account_result = read_instruments_account(&client);

        assert!(instruments_account_result.is_ok(), "Failed to read instruments account");

        let instruments_account = instruments_account_result.expect("Instruments account should be present");

        assert_eq!(instruments_account.header.tag, INSTR_ACCOUNT_TAG as u32);
        assert_eq!(instruments_account.header.version, INSTR_ACCOUNT_VERSION as u32);
    }
}
