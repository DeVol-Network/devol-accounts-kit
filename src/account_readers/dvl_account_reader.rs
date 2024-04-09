// use std::collections::HashMap;
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
    // cached_accounts: HashMap<Pubkey, Box<dyn DvlReadable>>,
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
            // cached_accounts: HashMap::new(),
        }
    }

    pub fn read<T: DvlReadable>(&self, id: Option<u32>) -> Result<T, Box<dyn Error>>
    {
        T::read(self, id)
    }

    pub fn read_indexed<T: DvlReadableIndexed>(&self, index: usize, id: Option<u32>) -> Result<T, Box<dyn Error>>
    {
        T::read(self, index, id)
    }

    pub fn read_by_public_key<T: DvlReadablePublicKey + DevolAccount + Copy>(&self, public_key: &Pubkey, id: Option<u32>) -> Result<T, Box<dyn Error>>
    {
        T::read_by_public_key::<T>(self, public_key, id)
    }


    // pub fn read_cached<T: DvlReadable + Clone>(&mut self, public_key: &Pubkey) -> Result<T, Box<dyn Error>> {
    //     if let Some(cached_value) = self.cached_accounts.get(public_key) {
    //         if let Some(value) = cached_value.as_any().downcast_ref::<T>() {
    //             return Ok(value.clone());
    //         }
    //     }
    //
    //     let value = T::read(self, public_key);
    //     self.cached_accounts.insert(public_key.to_string(), Box::new(value.clone()));
    //     Ok(value)
    // }
}