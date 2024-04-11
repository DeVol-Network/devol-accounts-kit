use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::devol_account::DevolAccount;

pub struct IndexedAccountParams {
    pub id: u32,
}
pub struct ClientRelativeAccountParams<'a> {
    pub client_account: &'a ClientAccount,
}
pub struct SignableAccountParams<'a> {
    pub client_address: &'a Pubkey,
    pub signer_account: Option<SignerAccountParams<'a>>,
}

pub struct SignerAccountParams<'a> {
    pub signer: &'a Pubkey,
    pub devol_sign: bool,
}

pub trait DvlReadable {
    type AdditionalCheckParams<'a>;

    fn read<'a>(reader: &DvlAccountReader, params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>>
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
