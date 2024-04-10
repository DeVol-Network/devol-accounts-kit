use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;

pub trait DvlReadablePublicKey {
    fn read_by_public_key(
        reader: &DvlAccountReader,
        public_key: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + DevolRegularAccount + Copy
    {
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            )?;
        Ok(account)
    }
    fn read_by_public_key(
        reader: &DvlAccountReader,
        public_key: &Pubkey,
        id: u32,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + DevolIndexedAccount + Copy
    {
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;
        Ok(account)
    }
}
pub trait DvlReadableIndexed {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;
}
pub trait DvlReadable {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;
}
pub trait DvlReadableClient {
    fn read(
        reader: &DvlAccountReader,
        account_address: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;
}

