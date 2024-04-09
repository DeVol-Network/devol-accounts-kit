use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{DvlReadableIndexed, DvlReadablePublicKey};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TasksTraceAccount;

impl DvlReadablePublicKey for TasksTraceAccount {}

impl DvlReadableIndexed for TasksTraceAccount {
    fn read(reader: &DvlAccountReader, index: usize, id: Option<u32>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let worker = workers_account.workers[index];
        let public_key = &worker.tasks_trace_address;
        let account =  Self::read_by_public_key(reader, public_key, id)?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::worker::tasks_trace::tasks_trace_account::{TASKS_TRACE_ACCOUNT_TAG, TASKS_TRACE_ACCOUNT_VERSION, TasksTraceAccount};
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_tasks_trace_account() {
        let reader = setup_account_reader();
        // Test read by index
        let task_trace_0 = reader.read_indexed::<TasksTraceAccount>(0,None).unwrap();
        check_tasks_trace_account(&task_trace_0);
        // Test read by public key
        let workers_account = reader.read::<AllWorkersAccount>(None).unwrap();
        let pubkey = &workers_account.workers[0].tasks_trace_address;
        let task_trace_0 = reader.read_by_public_key::<TasksTraceAccount>(pubkey,None).unwrap();
        check_tasks_trace_account(&task_trace_0);
    }

    fn check_tasks_trace_account(mint_log_account: &TasksTraceAccount){
        assert_eq!(mint_log_account.header.tag, TASKS_TRACE_ACCOUNT_TAG as u32);
        assert_eq!(mint_log_account.header.version, TASKS_TRACE_ACCOUNT_VERSION);
    }
}
