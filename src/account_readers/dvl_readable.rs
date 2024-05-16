use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::client::client_account::signer_account_params::SignerAccountParams;
use crate::dvl_client::dvl_client::DvlClient;
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

pub trait DvlParametrable : Sync + Send {
    type DvlReadParams<'a>: Sync;
}

#[async_trait]
pub trait DvlReadable : DvlParametrable {
    async fn get_public_key<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>>
        where
            Self: Sized + Send;

    async fn read<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>)
        -> Result<Box<Self>, Box<dyn Error>> where Self: Sized + DevolAccount + Copy + Send{
        let public_key = &*Self::get_public_key(client, params).await?;
        let mut rpc_data = client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &client.root_pda.key,
            &client.program_id,
            params,
        )?;
        Ok(account)
    }

    async fn read_by_public_key(
        client: &DvlClient,
        public_key: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + DevolAccount + Copy + Send
    {
        let mut rpc_data = client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account_basic(
            public_key,
            &mut rpc_data,
            &client.root_pda.key,
            &client.program_id,
        )?;

        Ok(account)
    }
}
