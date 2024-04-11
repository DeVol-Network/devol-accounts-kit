use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_account::DevolAccount;
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
    pub fn new(client: RpcClient, int_seed: usize, admin_public_key_str: &str, program_id_str: &str) -> Self {
        let admin_public_key = Pubkey::from_str(admin_public_key_str).unwrap();
        let program_id = Pubkey::from_str(program_id_str).unwrap();
        let root_seed = format!("rt{}", int_seed);
        let root_pda = generate_pda(&admin_public_key, &root_seed, &program_id);

        Self {
            client,
            int_seed,
            admin_public_key,
            program_id,
            main_seed: format!("cm{}", int_seed),
            log_seed: format!("pol{}", int_seed),
            root_seed,
            oracle_seed: format!("orcl{}", int_seed),
            root_pda,
        }
    }

    pub fn read<'a, T: DvlReadable>(&self, check: T::AdditionalCheckParams<'a>) -> Result<Box<T>, Box<dyn Error>> {
        T::read(self, check)
    }
    pub fn read_by_public_key<T: DvlReadable + DevolAccount + Copy>(
        &self,
        public_key: &Pubkey,
    ) -> Result<Box<T>, Box<dyn Error>> {
        T::read_by_public_key(self, public_key)
    }
}