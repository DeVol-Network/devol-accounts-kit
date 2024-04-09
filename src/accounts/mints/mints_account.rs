use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mint::Mint;

pub const MINTS_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const MINTS_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const MINTS_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const MINTS_ACCOUNT_DATA_OFFSET: usize = 44;
pub const MINTS_ACCOUNT_SIZE: usize = 4140;
pub const MINTS_ACCOUNT_TAG: u8 = 1;
pub const MINTS_ACCOUNT_VERSION: u32 = 3;
pub const MAX_MINTS_COUNT: usize = 32;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MintsAccount {
    pub header: AccountHeader, // 40 bytes
    pub count: u32, // 4 bytes
    pub data: [Mint; MAX_MINTS_COUNT],
}

impl DevolAccount for MintsAccount {
    #[inline(always)]
    fn expected_size() -> usize { MINTS_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 { MINTS_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { MINTS_ACCOUNT_VERSION }
}

impl Default for MintsAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            count: 0,
            data: [Mint::default(); MAX_MINTS_COUNT],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::accounts::mints::mint::{MINT_SIZE};

    #[test]
    fn test_root_account_offsets() {
        let account = MintsAccount::default();

        for i in 0..MAX_MINTS_COUNT {
            let mint = &account.data[i];
            let mint_ptr = mint as *const _;
            let expected_ptr = account.data.as_ptr() as usize + i * MINT_SIZE;
            assert_eq!(mint_ptr as usize, expected_ptr, "Mint offset is incorrect for index {}", i);
        }

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, MINTS_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, MINTS_ACCOUNT_COUNT_OFFSET);
        assert_eq!(unsafe { &account.data as *const _ as usize } - base_ptr, MINTS_ACCOUNT_DATA_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<MintsAccount>(), MINTS_ACCOUNT_SIZE);
    }
}
