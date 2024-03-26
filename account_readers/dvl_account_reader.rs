use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::generate_pda::{generate_pda, PDA};

pub struct DvlAccountReader {
    pub client: RpcClient,
    pub int_seed: usize,
    pub admin_public_key: Pubkey,
    pub program_id: Pubkey,
    pub main_seed: String,
    pub log_seed: String,
    pub root_seed: String,
    pub oracle_seed: String,
    pub root_pda: PDA,
}

impl DvlAccountReader {
    pub fn new(rpc_url: &str, int_seed: usize, admin_public_key_str: &str, program_id_str: &str) -> Result<Self, Box<dyn Error>> {
        let client = RpcClient::new(String::from(rpc_url));
        let admin_public_key = Pubkey::from_str(admin_public_key_str)?;
        let program_id = Pubkey::from_str(program_id_str)?;
        let root_seed = format!("rt{}", int_seed);
        let root_pda = generate_pda(&admin_public_key, &root_seed, &program_id)?;

        Ok(Self {
            client,
            int_seed,
            admin_public_key,
            program_id,
            main_seed: format!("cm{}", int_seed),
            log_seed: format!("pol{}", int_seed),
            root_seed,
            oracle_seed: format!("orcl{}", int_seed),
            root_pda,
        })
    }
}