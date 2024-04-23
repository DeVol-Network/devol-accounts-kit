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
        let mut rpc_data = reader.client.get_account(public_key)?;
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
    use crate::account_readers::dvl_readable::DvlIndexParam;
    use crate::accounts::mints::mint_log::mint_log_account::{*};
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_mint_log_account() {
        let reader = setup_account_reader();
        // Test read by index
        let mint_log_0 = reader.get_account::<MintLogAccount>(DvlIndexParam {id: 0}).unwrap();
        check_mint_log_account(&mint_log_0);
        // Test read by public key
        let mints_account = reader.get_account::<MintsAccount>(()).unwrap();
        let pubkey = &mints_account.data[0].log_address;
        let mint_log_0 = reader.get_account_by_public_key::<MintLogAccount>(pubkey).unwrap();
        check_mint_log_account(&mint_log_0);
    }

    fn check_mint_log_account(mint_log_account: &MintLogAccount){
        assert_eq!(mint_log_account.header.tag, MINT_LOG_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, MINT_LOG_ACCOUNT_VERSION);
    }
}