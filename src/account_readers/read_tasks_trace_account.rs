use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable, DvlIndexParam};
use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TasksTraceAccount;

impl DvlReadable for TasksTraceAccount {
    type DvlReadParams<'a> = DvlIndexParam;

    fn get_public_key<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let worker = workers_account.workers[params.id as usize];
        Ok(Box::from(worker.tasks_trace_address))
    }

    fn read<'a>(reader: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(reader, params)?;
        let mut rpc_data = reader.rpc_client.get_account(public_key)?;
        let account =  Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            Some(params.id),
        )?;
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
        let task_trace_0 = reader.get_account::<TasksTraceAccount>(DvlIndexParam {id: 0}).unwrap();
        check_tasks_trace_account(&task_trace_0);
        // Test read by public key
        let workers_account = reader.get_account::<AllWorkersAccount>(()).unwrap();
        let pubkey = &workers_account.workers[0].tasks_trace_address;
        let task_trace_0 = reader.get_account_by_public_key::<TasksTraceAccount>(pubkey).unwrap();
        check_tasks_trace_account(&task_trace_0);
    }

    fn check_tasks_trace_account(tasks_trace_account: &TasksTraceAccount){
        assert_eq!(tasks_trace_account.header.tag, TASKS_TRACE_ACCOUNT_TAG as u32);
        assert_eq!(tasks_trace_account.header.version, TASKS_TRACE_ACCOUNT_VERSION);
    }
}
