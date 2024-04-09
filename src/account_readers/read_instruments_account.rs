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
        let pubkey = &root.instruments_address;
        let mut rpc_data = reader.client.get_account(pubkey)?;
        let account = InstrumentsAccount::from_account(
            pubkey,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::instruments::instruments_account::{INSTR_ACCOUNT_TAG, INSTR_ACCOUNT_VERSION, InstrumentsAccount};
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_root_account() {
        let reader = setup_account_reader();
        let instruments_account = reader.read::<InstrumentsAccount>(None).unwrap();

        assert_eq!(instruments_account.header.tag, INSTR_ACCOUNT_TAG as u32);
        assert_eq!(instruments_account.header.version, INSTR_ACCOUNT_VERSION as u32);
    }
}