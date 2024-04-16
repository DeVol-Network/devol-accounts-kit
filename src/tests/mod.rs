#[cfg(feature = "off-chain")]
#[cfg(test)]
pub mod tests {
    use solana_client::rpc_client::RpcClient;
    use crate::dvl_interact::dvl_interact::DvlInteract;
    use crate::constants::test_constants::*;

    pub fn setup_account_reader() -> DvlInteract {
        let client = RpcClient::new(String::from(RPC_URL));
        let reader = DvlInteract::new(client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
        reader
    }
}