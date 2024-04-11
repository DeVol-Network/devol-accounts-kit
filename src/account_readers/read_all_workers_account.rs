use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for AllWorkersAccount {
    type AccountParam = ();

    fn read(reader: &DvlAccountReader, _params: Self::AccountParam) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let root = reader.read::<RootAccount>(()).unwrap();
        let public_key = &root.workers_address;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account =  Self::from_account(
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
    use crate::accounts::all_workers::all_workers_account::{ALL_WORKERS_ACCOUNT_TAG, ALL_WORKERS_ACCOUNT_VERSION, AllWorkersAccount};
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_all_workers_account() {
        let reader = setup_account_reader();
        // Test auto read
        let all_workers_account = reader.read::<AllWorkersAccount>(()).unwrap();
        check_all_workers_account(&all_workers_account);
        // Test read by public key
        // let root_account = reader.read::<RootAccount>(()).unwrap();
        // let pubkey = &root_account.workers_address;
        // let all_workers_account = reader.read_by_public_key::<AllWorkersAccount>(pubkey,None).unwrap();
        // check_all_workers_account(&all_workers_account);
    }

    fn check_all_workers_account(all_workers_account: &AllWorkersAccount){
        assert_eq!(all_workers_account.header.tag, ALL_WORKERS_ACCOUNT_TAG as u32);
        assert_eq!(all_workers_account.header.version, ALL_WORKERS_ACCOUNT_VERSION as u32);
    }
}
