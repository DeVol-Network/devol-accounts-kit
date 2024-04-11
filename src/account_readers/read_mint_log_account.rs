use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, IndexedAccountParams};
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
use crate::accounts::mints::mints_account::MintsAccount;


impl DvlReadable for MintLogAccount {
    type AdditionalCheckParams = IndexedAccountParams;

    fn read(reader: &DvlAccountReader, params: Self::AdditionalCheckParams) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let mints_account = reader.read::<MintsAccount>(()).unwrap();
        let public_key = &mints_account.data[params.id].log_address;
        // let account =  Self::read_by_public_key(reader, public_key, params)?;
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
    use crate::account_readers::dvl_readable::IndexedAccountParams;
    use crate::accounts::mints::mint_log::mint_log_account::{*};
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_mint_log_account() {
        let reader = setup_account_reader();
        // Test read by index
        let mint_log_0 = reader.read::<MintLogAccount>(IndexedAccountParams {id: 0}).unwrap();
        check_mint_log_account(&mint_log_0);
        // Test read by public key
        let mints_account = reader.read::<MintsAccount>(()).unwrap();
        let pubkey = &mints_account.data[0].log_address;
        let mint_log_0 = reader.read_by_public_key::<MintLogAccount>(pubkey).unwrap();
        check_mint_log_account(&mint_log_0);
    }

    fn check_mint_log_account(mint_log_account: &MintLogAccount){
        assert_eq!(mint_log_account.header.tag, MINT_LOG_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, MINT_LOG_ACCOUNT_VERSION);
    }
}