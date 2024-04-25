use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::generate_pda::generate_pda;


impl DvlReadable for OraclesAccount {
    type DvlReadParams<'a> = ();

    fn get_public_key<'a>(reader: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let oracle_seed = format!("{}{}", reader.oracle_seed, reader.int_seed);
        let oracle_pda = generate_pda(&reader.admin_public_key, &oracle_seed, &reader.program_id);
        Ok(Box::from(oracle_pda.key))
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
    use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_TAG, ORACLES_ACCOUNT_VERSION, OraclesAccount};
    use crate::generate_pda::generate_pda;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_oracles_account() {
        let reader = setup_account_reader();
        // Test read by public key
        let oracle_pda = generate_pda(&reader.admin_public_key, &reader.oracle_seed, &reader.program_id);
        let pubkey = &oracle_pda.key;
        let oracles_account = reader.get_account_by_public_key::<OraclesAccount>(pubkey).unwrap();
        check_oracles_account(&oracles_account);
    }

    fn check_oracles_account(oracles_account: &OraclesAccount){
        assert_eq!(oracles_account.header.tag, ORACLES_ACCOUNT_TAG as u32);
        assert_eq!(oracles_account.header.version, ORACLES_ACCOUNT_VERSION);
    }
}
