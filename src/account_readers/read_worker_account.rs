use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::worker::worker_account::WorkerAccount;

impl DvlReadablePublicKey for WorkerAccount {}

impl DvlReadableIndexed for WorkerAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let worker = workers_account.workers[index];
        let public_key = &worker.address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::worker::worker_account::{WORKER_ACCOUNT_TAG, WORKER_ACCOUNT_VERSION};
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_worker_account() {
        let reader = setup_account_reader();
        // Test read by index
        let worker_0 = reader.read_indexed::<WorkerAccount>(0,None).unwrap();
        check_worker_account(&worker_0);
        // Test read by public key
        let mints_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let pubkey = &mints_account.workers[0].address;
        let worker_0 = reader.read_by_public_key::<WorkerAccount>(pubkey,None).unwrap();
        check_worker_account(&worker_0);
    }

    fn check_worker_account(mint_log_account: &WorkerAccount){
        assert_eq!(mint_log_account.header.tag, WORKER_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, WORKER_ACCOUNT_VERSION);
    }
}
