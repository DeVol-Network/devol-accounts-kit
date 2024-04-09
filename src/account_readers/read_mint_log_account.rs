use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
use crate::accounts::mints::mints_account::MintsAccount;

impl DvlReadablePublicKey for MintLogAccount {}

impl DvlReadableIndexed for MintLogAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let mints_account = reader.read::<MintsAccount>(None).unwrap();
        let public_key = &mints_account.data[index].log_address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::mints::mint_log::mint_log_account::{*};
    use crate::accounts::mints::mints_account::MintsAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_mint_log_account() {
        let reader = setup_account_reader();
        // Test read by index
        let mint_log_0 = reader.read_indexed::<MintLogAccount>(0,None).unwrap();
        check_mint_log_account(&mint_log_0);
        // Test read by public key
        let mints_account = reader.read::<MintsAccount>(None).unwrap();
        let pubkey = &mints_account.data[0].log_address;
        let mint_log_0 = reader.read_by_public_key::<MintLogAccount>(pubkey,None).unwrap();
        check_mint_log_account(&mint_log_0);
    }

    fn check_mint_log_account(mint_log_account: &MintLogAccount){
        assert_eq!(mint_log_account.header.tag, MINT_LOG_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, MINT_LOG_ACCOUNT_VERSION);
    }
}