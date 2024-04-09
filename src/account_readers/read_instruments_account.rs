use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadablePublicKey};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadablePublicKey for InstrumentsAccount {}

impl DvlReadable for InstrumentsAccount {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let root = reader.read::<RootAccount>(None).unwrap();
        let public_key = &root.instruments_address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::instruments::instruments_account::{INSTR_ACCOUNT_TAG, INSTR_ACCOUNT_VERSION, InstrumentsAccount};
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_instruments_account() {
        let reader = setup_account_reader();
        // Test auto read
        let instruments_account = reader.read::<InstrumentsAccount>(None).unwrap();
        check_instruments_account(&instruments_account);
        // Test read by public key
        let root_account = reader.read::<RootAccount>(None).unwrap();
        let pubkey = &root_account.instruments_address;
        let instruments_account = reader.read_by_public_key::<InstrumentsAccount>(pubkey,None).unwrap();
        check_instruments_account(&instruments_account);
    }

    fn check_instruments_account(instruments_account: &InstrumentsAccount){
        assert_eq!(instruments_account.header.tag, INSTR_ACCOUNT_TAG as u32);
        assert_eq!(instruments_account.header.version, INSTR_ACCOUNT_VERSION);
    }
}