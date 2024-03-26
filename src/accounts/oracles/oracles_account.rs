use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::oracles::oracles_data::{OracleData, ORACLES_DATA_COUNT};

pub const ORACLES_ACCOUNT_HEADER_OFFSET: usize = 0;
pub const ORACLES_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const ORACLES_ACCOUNT_RESERVED_OFFSET: usize = 44;
pub const ORACLES_ACCOUNT_DATA_OFFSET: usize = 48;
pub const ORACLES_ACCOUNT_SIZE: usize = 1776;
pub const ORACLES_ACCOUNT_TAG: u8 = 13;
pub const ORACLES_ACCOUNT_VERSION: usize = 1;

#[repr(C)]
pub struct OraclesAccount {
    pub header: AccountHeader,                      // ORACLES_ACCOUNT_HEADER_OFFSET
    pub count: u32,                                 // ORACLES_ACCOUNT_COUNT_OFFSET
    pub reserved: u32,                              // ORACLES_ACCOUNT_RESERVED_OFFSET
    pub data: [OracleData; ORACLES_DATA_COUNT],     // ORACLES_ACCOUNT_DATA_OFFSET
}

impl From<&[u8]> for OraclesAccount {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), ORACLES_ACCOUNT_SIZE, "Incorrect oracles account size.");
        unsafe {
            std::ptr::read_unaligned(bytes.as_ptr() as *const OraclesAccount)
        }
    }
}

impl DevolAccount for OraclesAccount {
    fn expected_size() -> usize { ORACLES_ACCOUNT_SIZE }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_oracles_account_offsets_and_size() {

        let account = OraclesAccount {
            header: AccountHeader::default(),
            count: 0,
            reserved: 0,
            data: [OracleData::default(); ORACLES_DATA_COUNT],
        };

        let base_ptr = &account as *const _ as usize;

        // Check offsets
        // Assuming we have constants for offsets
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, ORACLES_ACCOUNT_HEADER_OFFSET, "Header offset mismatch");
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, ORACLES_ACCOUNT_COUNT_OFFSET, "Count offset mismatch");
        assert_eq!(unsafe { &account.reserved as *const _ as usize } - base_ptr, ORACLES_ACCOUNT_RESERVED_OFFSET, "Reserved offset mismatch");
        assert_eq!(unsafe { &account.data as *const _ as usize } - base_ptr, ORACLES_ACCOUNT_DATA_OFFSET, "Data array offset mismatch");

        // Check total size of the OraclesAccount structure
        assert_eq!(mem::size_of::<OraclesAccount>(), ORACLES_ACCOUNT_SIZE, "Total size of OraclesAccount structure mismatch");
    }
}
