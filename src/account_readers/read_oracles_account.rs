use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadable, DvlReadablePublicKey};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::constants::test_constants::{INT_SEED, ORACLE_SEED_PREFIX};
use crate::generate_pda::generate_pda;

impl DvlReadablePublicKey for OraclesAccount {}

impl DvlReadable for OraclesAccount {
    fn read(reader: &DvlAccountReader, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let oracle_seed = format!("{}{}", ORACLE_SEED_PREFIX, INT_SEED);
        let oracle_pda = generate_pda(&reader.admin_public_key, &oracle_seed, &reader.program_id);
        let pubkey = &oracle_pda.key;
        let mut rpc_data = reader.client.get_account(pubkey)?;
        let oracles_account = OraclesAccount::from_account(
            pubkey,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            id)?;

        Ok(oracles_account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_TAG, ORACLES_ACCOUNT_VERSION, OraclesAccount};
    use crate::constants::test_constants::{INT_SEED, ORACLE_SEED_PREFIX};
    use crate::generate_pda::generate_pda;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_oracles_account() {
        let reader = setup_account_reader();
        // Test auto read
        let oracles_account = reader.read::<OraclesAccount>(None).unwrap();
        check_oracles_account(&oracles_account);
        // Test read by public key
        let oracle_seed = format!("{}{}", ORACLE_SEED_PREFIX, INT_SEED);
        let oracle_pda = generate_pda(&reader.admin_public_key, &oracle_seed, &reader.program_id);
        let pubkey = &oracle_pda.key;
        let oracles_account = reader.read_by_public_key::<OraclesAccount>(pubkey,None).unwrap();
        check_oracles_account(&oracles_account);
    }

    fn check_oracles_account(oracles_account: &OraclesAccount){
        assert_eq!(oracles_account.header.tag, ORACLES_ACCOUNT_TAG as u32);
        assert_eq!(oracles_account.header.version, ORACLES_ACCOUNT_VERSION);
    }
}
