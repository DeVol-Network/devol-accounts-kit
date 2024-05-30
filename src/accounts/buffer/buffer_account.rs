use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;

pub const BUFFER_ACCOUNT_TAG: u8 = 14;
pub const BUFFER_ACCOUNT_MIN_SIZE: usize = 40;
pub const BUFFER_ACCOUNT_VERSION: u8 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferAccount {
    pub header: AccountHeader, // 40 bytes
}

impl DevolAccount for BufferAccount {
    fn expected_size() -> usize { BUFFER_ACCOUNT_MIN_SIZE }

    fn expected_tag() -> u8 {
        BUFFER_ACCOUNT_TAG
    }

    fn expected_version() -> u32 { BUFFER_ACCOUNT_VERSION as u32 }
}
