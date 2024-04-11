use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;

pub struct IndexedAccountParams {
    pub id: usize,
}
pub struct SignableAccountParams {
    pub client_address: Box<Pubkey>,
    pub signer: Box<Pubkey>,
    pub devol_sign: bool,
}

pub trait DvlReadable {
    type AdditionalCheckParams;

    fn read(reader: &DvlAccountReader, params: Self::AdditionalCheckParams) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;

    fn read_by_public_key(
        reader: &DvlAccountReader,
        public_key: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + DevolAccount + Copy
    {
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account_basic(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
        )?;

        Ok(account)
    }
}
