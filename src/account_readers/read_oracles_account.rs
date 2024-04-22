use std::error::Error;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::generate_pda::generate_pda;


impl DvlReadable for OraclesAccount {
    type AdditionalCheckParams<'a> = ();

    fn read<'a>(reader: &DvlClient, _params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let oracle_seed = format!("{}{}", reader.oracle_seed, reader.int_seed);
        let oracle_pda = generate_pda(&reader.admin_public_key, &oracle_seed, &reader.program_id);
        let public_key = &oracle_pda.key;
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
    use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_TAG, ORACLES_ACCOUNT_VERSION, OraclesAccount};
    use crate::generate_pda::generate_pda;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_oracles_account() {
        let reader = setup_account_reader();
        // Test read by public key
        let oracle_pda = generate_pda(&reader.admin_public_key, &reader.oracle_seed, &reader.program_id);
        let pubkey = &oracle_pda.key;
        let oracles_account = reader.read_by_public_key::<OraclesAccount>(pubkey).unwrap();
        check_oracles_account(&oracles_account);
    }

    fn check_oracles_account(oracles_account: &OraclesAccount){
        assert_eq!(oracles_account.header.tag, ORACLES_ACCOUNT_TAG as u32);
        assert_eq!(oracles_account.header.version, ORACLES_ACCOUNT_VERSION);
    }
}
