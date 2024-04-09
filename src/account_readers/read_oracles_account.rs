use crate::account_readers::dvl_readable::{DvlReadablePublicKey};
use crate::accounts::oracles::oracles_account::OraclesAccount;

impl DvlReadablePublicKey for OraclesAccount {}

#[cfg(test)]
mod tests {
    use crate::accounts::oracles::oracles_account::{ORACLES_ACCOUNT_TAG, ORACLES_ACCOUNT_VERSION, OraclesAccount};
    use crate::constants::test_constants::{INT_SEED, ORACLE_SEED_PREFIX};
    use crate::generate_pda::generate_pda;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_oracles_account() {
        let reader = setup_account_reader();
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
