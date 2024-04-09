use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::devol_account::DevolAccount;

pub trait DvlReadablePublicKey {
    fn read_by_public_key<T: DevolAccount>(
        reader: &DvlAccountReader,
        public_key: &Pubkey,
        id: Option<u32>,
    ) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized;
    // {
        // let mut rpc_data = reader.client.get_account(public_key)?;
        // let account : T =  T::from_account(
        //     public_key,
        //     &mut rpc_data,
        //     &reader.root_pda.key,
        //     &reader.program_id,
        //     id)?;
        // Ok(account)
    // }
}
pub trait DvlReadableIndexed {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized;
}
pub trait DvlReadable {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized;
}

