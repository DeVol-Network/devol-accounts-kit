use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for InstrumentsAccount {
    type DvlReadParams<'a> = ();

    fn get_public_key<'a>(reader: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let root = reader.get_account::<RootAccount>(()).unwrap();
        Ok(Box::from(root.instruments_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::instruments::instruments_account::InstrumentsAccount;
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_instruments_account_auto() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let _instruments_account = reader.get_account::<InstrumentsAccount>(())?;
        Ok(())
    }

    #[test]
    fn test_read_instruments_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let reader = setup_devol_client();
        let root_account = reader.get_account::<RootAccount>(())?;
        let pubkey = &root_account.instruments_address;
        let _instruments_account = reader.get_account_by_public_key::<InstrumentsAccount>(pubkey)?;
        Ok(())
    }
}
