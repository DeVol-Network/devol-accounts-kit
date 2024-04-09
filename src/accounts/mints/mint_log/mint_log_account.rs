use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mint_log::mint_log::MintLog;

pub const MINT_LOG_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const MINT_LOG_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const MINT_LOG_ACCOUNT_MINTS_ADDRESS_OFFSET: usize = 40;
pub const MINT_LOG_ACCOUNT_MINT_ID_OFFSET: usize = 72;
pub const MINT_LOG_ACCOUNT_LAST_OFFSET: usize = 76;
pub const MINT_LOG_ACCOUNT_COUNT_OFFSET: usize = 80;
pub const MINT_LOG_ACCOUNT_DATA_OFFSET: usize = 84;
pub const MINT_LOG_ACCOUNT_SIZE: usize = 19540;
pub const MINT_LOG_ACCOUNT_TAG: u8 = 10;
pub const MINT_LOG_ACCOUNT_VERSION: u32 = 12;
pub const MINT_LOG_BUFFER_CAPACITY: usize = 256;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MintLogAccount {
    pub header: AccountHeader,
    // 40 bytes
    pub mints_address: Pubkey,
    pub mint_id: u32,
    pub last: u32,
    pub count: u32,
    pub data: [MintLog; MINT_LOG_BUFFER_CAPACITY],
}

impl DevolAccount for MintLogAccount {
    #[inline(always)]
    fn expected_size() -> usize { MINT_LOG_ACCOUNT_SIZE }
    #[inline(always)]
    fn expected_tag() -> u8 { MINT_LOG_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { MINT_LOG_ACCOUNT_VERSION }
}

impl Default for MintLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            mints_address: Pubkey::default(),
            mint_id: 0,
            last: 0,
            count: 0,
            data: [MintLog::default(); MINT_LOG_BUFFER_CAPACITY],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::accounts::mints::mint_log::mint_log::MINT_LOG_SIZE;

    #[test]
    fn test_root_account_offsets() {
        let account = MintLogAccount::default();

        for i in 0..MINT_LOG_BUFFER_CAPACITY {
            let mint = &account.data[i];
            let mint_ptr = mint as *const _;
            let expected_ptr = account.data.as_ptr() as usize + i * MINT_LOG_SIZE;
            assert_eq!(mint_ptr as usize, expected_ptr, "Mint offset is incorrect for index {}", i);
        }

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.mints_address as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_MINTS_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.mint_id as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_MINT_ID_OFFSET);
        assert_eq!(unsafe { &account.last as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_LAST_OFFSET);
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_COUNT_OFFSET);
        assert_eq!(unsafe { &account.data as *const _ as usize } - base_ptr, MINT_LOG_ACCOUNT_DATA_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<MintLogAccount>(), MINT_LOG_ACCOUNT_SIZE);
    }
}
