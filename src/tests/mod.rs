mod platform_test;

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
pub(crate) mod tests {
    use std::str::FromStr;
    use solana_client::rpc_client::RpcClient;
    use solana_program::pubkey::Pubkey;
    use crate::dvl_client::dvl_client::DvlClient;
    use crate::constants::test_constants::*;

    pub fn setup_devol_client() -> DvlClient {
        let client = RpcClient::new(String::from(RPC_URL));
        let admin_pub_key = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let reader = DvlClient::new(client, INT_SEED, admin_pub_key, program_id);
        reader
    }
}