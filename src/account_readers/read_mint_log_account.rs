use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
use crate::accounts::mints::mints_account::MintsAccount;


impl DvlReadable for MintLogAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    fn get_public_key<'a>(reader: &DvlClient, params: &DvlIndexParam) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let mints_account = reader.get_account::<MintsAccount>(()).unwrap();
        Ok(Box::from(mints_account.data[params.id as usize].log_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            Some(params.id),
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_readers::dvl_readable::DvlIndexParam;
    use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_mint_log_account_by_index() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let _mint_log_0 = reader.get_account::<MintLogAccount>(DvlIndexParam { id: 0 })?;
        Ok(())
    }

    #[test]
    fn test_read_mint_log_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let mints_account = reader.get_account::<MintsAccount>(())?;
        let pubkey = &mints_account.data[0].log_address;
        let _mint_log_0 = reader.get_account_by_public_key::<MintLogAccount>(pubkey)?;
        Ok(())
    }
}