#[cfg(feature = "off-chain")]
#[cfg(test)]
pub mod tests {
    use solana_client::rpc_client::RpcClient;
    use crate::account_readers::dvl_account_reader::DvlAccountReader;
    use crate::constants::test_constants::*;

    pub fn setup_account_reader() -> DvlAccountReader {
        let client = RpcClient::new(String::from(RPC_URL));
        let reader = DvlAccountReader::new(client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
        reader
    }
}