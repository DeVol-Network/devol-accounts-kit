use solana_program::pubkey::Pubkey;
use crate::accounts::all_workers::worker::Worker;

pub const ALL_WORKERS_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const ALL_WORKERS_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const ALL_WORKERS_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const ALL_WORKERS_ACCOUNT_DATA_OFFSET: usize = 44;
pub const ALL_WORKERS_ACCOUNT_SIZE: usize = 5164;
pub const ALL_WORKERS_ACCOUNT_TAG: u8 = 3;
pub const ALL_WORKERS_ACCOUNT_VERSION: usize = 5;
pub const MAX_ALL_WORKERS_COUNT: usize = 32;

#[repr(C)]
pub struct AllWorkersAccount {
    pub version: i64,
    pub root_address: Pubkey,
    pub count: u32,
    pub workers: [Worker; MAX_ALL_WORKERS_COUNT],
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::accounts::all_workers::worker::WORKER_SIZE;

    #[test]
    fn test_all_workers_account_offsets_and_size() {

        let account = AllWorkersAccount {
            version: 0,
            root_address: Pubkey::default(),
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
        assert_eq!(unsafe { &account.version as *const _ as usize } - base_ptr, ALL_WORKERS_ACCOUNT_VERSION_OFFSET, "Version offset mismatch");
        assert_eq!(unsafe { &account.root_address as *const _ as usize } - base_ptr, ALL_WORKERS_ACCOUNT_ROOT_ADDRESS_OFFSET, "Root address offset mismatch");
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, ALL_WORKERS_ACCOUNT_COUNT_OFFSET, "Count offset mismatch");
        assert_eq!(unsafe { &account.workers as *const _ as usize } - base_ptr, ALL_WORKERS_ACCOUNT_DATA_OFFSET, "Data array offset mismatch");

        let expected_size = (ALL_WORKERS_ACCOUNT_SIZE + 7) & !7;
        // Check total size of the AllWorkersAccount structure
        assert_eq!(mem::size_of::<AllWorkersAccount>(), expected_size, "Total size of AllWorkersAccount structure mismatch");
    }
}
