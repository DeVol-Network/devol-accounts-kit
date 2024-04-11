use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;

pub struct IndexedAccountParams {
    pub id: usize,
}

pub trait DvlReadable {
    type AccountParam;

    fn read(reader: &DvlAccountReader, params: Self::AccountParam) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;
}
