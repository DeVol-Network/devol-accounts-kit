use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::root::root_account::RootAccount;

impl DvlReadable for AllWorkersAccount {
    type DvlReadParams<'a> = ();

    fn get_public_key<'a>(reader: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>>
        where Self: Sized {
        let root = reader.get_account::<RootAccount>(()).unwrap();
        Ok(Box::from(root.workers_address))
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
    use crate::accounts::all_workers::all_workers_account::{ALL_WORKERS_ACCOUNT_TAG, ALL_WORKERS_ACCOUNT_VERSION, AllWorkersAccount};
    use crate::accounts::root::root_account::RootAccount;
    use crate::tests::tests::setup_devol_client;

    #[test]
    fn test_read_all_workers_account() {
        let reader = setup_devol_client();
        // Test auto read
        let all_workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        check_all_workers_account(&all_workers_account);
        // Test read by public key
        let root_account = reader.get_account::<RootAccount>(()).unwrap();
        let pubkey = &root_account.workers_address;
        let all_workers_account = reader.get_account_by_public_key::<AllWorkersAccount>(pubkey).unwrap();
        check_all_workers_account(&all_workers_account);
    }

    fn check_all_workers_account(all_workers_account: &AllWorkersAccount) {
        assert_eq!(all_workers_account.header.tag, ALL_WORKERS_ACCOUNT_TAG as u32);
        assert_eq!(all_workers_account.header.version, ALL_WORKERS_ACCOUNT_VERSION as u32);
    }
}
