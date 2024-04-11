use crate::accounts::account_header::AccountHeader;
use crate::accounts::all_workers::worker::Worker;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;

pub const ALL_WORKERS_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const ALL_WORKERS_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const ALL_WORKERS_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const ALL_WORKERS_ACCOUNT_DATA_OFFSET: usize = 44;
pub const ALL_WORKERS_ACCOUNT_SIZE: usize = 5164;
pub const ALL_WORKERS_ACCOUNT_TAG: u8 = 3;
pub const ALL_WORKERS_ACCOUNT_VERSION: usize = 5;
pub const MAX_ALL_WORKERS_COUNT: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AllWorkersAccount {
    pub header: AccountHeader, // 40 bytes
    pub count: u32,
    pub workers: [Worker; MAX_ALL_WORKERS_COUNT],
}

impl DevolRegularAccount for AllWorkersAccount {}

impl DevolAccount for AllWorkersAccount {
    fn expected_size() -> usize { ALL_WORKERS_ACCOUNT_SIZE }

    fn expected_tag() -> u8 {
        ALL_WORKERS_ACCOUNT_TAG
    }

    fn expected_version() -> u32 { ALL_WORKERS_ACCOUNT_VERSION as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::accounts::all_workers::worker::WORKER_SIZE;
    use crate::accounts::account_header::AccountHeader;

    #[test]
    fn test_all_workers_account_offsets_and_size() {

        let account = AllWorkersAccount {
            header: AccountHeader::default(),
            count: 0,
            workers: [Worker::default(); MAX_ALL_WORKERS_COUNT],
        };

        for i in 0..MAX_ALL_WORKERS_COUNT {
            let worker = &account.workers[i];
            let worker_ptr = worker as *const _;
            let expected_ptr = account.workers.as_ptr() as usize + i * WORKER_SIZE;
            assert_eq!(worker_ptr as usize, expected_ptr, "Worker offset is incorrect for index {}", i);
        }

        let base_ptr = &account as *const _ as usize;

        // Check offsets
        assert_eq!(&account.header as *const _ as usize - base_ptr, ALL_WORKERS_ACCOUNT_VERSION_OFFSET, "Header offset mismatch");
        assert_eq!(&account.count as *const _ as usize - base_ptr, ALL_WORKERS_ACCOUNT_COUNT_OFFSET, "Count offset mismatch");
        assert_eq!(&account.workers as *const _ as usize - base_ptr, ALL_WORKERS_ACCOUNT_DATA_OFFSET, "Data array offset mismatch");

        // Check total size of the AllWorkersAccount structure
        assert_eq!(mem::size_of::<AllWorkersAccount>(), ALL_WORKERS_ACCOUNT_SIZE, "Total size of AllWorkersAccount structure mismatch");
    }
}
