use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::client::client_account::signer_account_params::SignerAccountParams;
use crate::accounts::devol_account::DevolAccount;

pub struct DvlIndexParam {
    pub id: u32,
}

pub struct DvlClientParam<'a> {
    pub client_account: &'a ClientAccount,
}

pub struct DvlClientParams<'a> {
    pub client_address: &'a Pubkey,
    pub signer_account_params: Option<&'a SignerAccountParams<'a>>,
}

pub trait DvlReadable {
    type DvlReadParams<'a>;

    fn get_public_key<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>>
        where
            Self: Sized;

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized;

    fn read_by_public_key(
        reader: &DvlClient,
        public_key: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + DevolAccount + Copy
    {
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account_basic(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
        )?;

        Ok(account)
    }
}
