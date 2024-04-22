mod platform_test;

#[cfg(feature = "off-chain")]
#[cfg(test)]
pub mod tests {
    use solana_client::rpc_client::RpcClient;
    use crate::dvl_client::dvl_client::DvlClient;
    use crate::constants::test_constants::*;

    pub fn setup_account_reader() -> DvlClient {
        let client = RpcClient::new(String::from(RPC_URL));
        let reader = DvlClient::new(client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
        reader
    }
}