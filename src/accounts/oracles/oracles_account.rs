use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::oracles::oracles_data::{OracleData, ORACLES_DATA_COUNT};

pub const ORACLES_ACCOUNT_HEADER_OFFSET: usize = 0;
pub const ORACLES_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const ORACLES_ACCOUNT_RESERVED_OFFSET: usize = 44;
pub const ORACLES_ACCOUNT_DATA_OFFSET: usize = 48;
pub const ORACLES_ACCOUNT_SIZE: usize = 1776;
pub const ORACLES_ACCOUNT_TAG: u8 = 13;
pub const ORACLES_ACCOUNT_VERSION: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OraclesAccount {
    pub header: AccountHeader,                      // ORACLES_ACCOUNT_HEADER_OFFSET
    pub count: u32,                                 // ORACLES_ACCOUNT_COUNT_OFFSET
    pub reserved: u32,                              // ORACLES_ACCOUNT_RESERVED_OFFSET
    pub data: [OracleData; ORACLES_DATA_COUNT],     // ORACLES_ACCOUNT_DATA_OFFSET
}

impl DevolRegularAccount for OraclesAccount {}
impl DevolAccount for OraclesAccount {
    #[inline(always)]
    fn expected_size() -> usize { ORACLES_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 {
        ORACLES_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 {
        ORACLES_ACCOUNT_VERSION
    }
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
        assert_eq!(&account.header as *const _ as usize - base_ptr, ORACLES_ACCOUNT_HEADER_OFFSET, "Header offset mismatch");
        assert_eq!(&account.count as *const _ as usize - base_ptr, ORACLES_ACCOUNT_COUNT_OFFSET, "Count offset mismatch");
        assert_eq!(&account.reserved as *const _ as usize - base_ptr, ORACLES_ACCOUNT_RESERVED_OFFSET, "Reserved offset mismatch");
        assert_eq!(&account.data as *const _ as usize - base_ptr, ORACLES_ACCOUNT_DATA_OFFSET, "Data array offset mismatch");

        // Check total size of the OraclesAccount structure
        assert_eq!(mem::size_of::<OraclesAccount>(), ORACLES_ACCOUNT_SIZE, "Total size of OraclesAccount structure mismatch");
    }
}
