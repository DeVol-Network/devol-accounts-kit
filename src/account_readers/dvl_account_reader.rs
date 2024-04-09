use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadableIndexed, DvlReadablePublicKey};
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

    pub fn read<T: DvlReadable>(&self, id: Option<u32>) -> Result<Box<T>, Box<dyn Error>>
    {
        T::read(self, id)
    }

    pub fn read_indexed<T: DvlReadableIndexed>(&self, index: usize, id: Option<u32>) -> Result<Box<T>, Box<dyn Error>>
    {
        T::read(self, index, id)
    }

    pub fn read_by_public_key<T: DvlReadablePublicKey>(
        &self,
        public_key: &Pubkey,
        id: Option<u32>,
    ) -> Result<Box<T>, Box<dyn Error>>
    where
        T: DevolAccount + Copy
    {
        T::read_by_public_key(self, public_key, id)
    }
}