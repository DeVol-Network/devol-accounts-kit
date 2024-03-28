use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::instruments::instruments_data::InstrumentsData;

pub const INSTR_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const INSTR_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const INSTR_ACCOUNT_COUNT_OFFSET: usize = 40;
pub const INSTR_ACCOUNT_DATA_OFFSET: usize = 44;
pub const INSTR_ACCOUNT_SIZE: usize = 6316;
pub const INSTR_DATA_COUNT: usize = 32;
pub const INSTR_ACCOUNT_TAG: u8 = 2;
pub const INSTR_ACCOUNT_VERSION: u32 = 4;

#[repr(C)]
pub struct InstrumentsAccount {
    pub header: AccountHeader,                        // INSTR_ACCOUNT_VERSION_OFFSET
    pub count: u32,                                   // INSTR_ACCOUNT_COUNT_OFFSET
    pub data: [InstrumentsData; INSTR_DATA_COUNT],    // INSTR_ACCOUNT_DATA_OFFSET
}

impl DevolAccount for InstrumentsAccount {
    fn expected_size() -> usize { INSTR_ACCOUNT_SIZE }

    fn expected_tag() -> u8 {
        INSTR_ACCOUNT_TAG
    }

    fn expected_version() -> u32 {
        INSTR_ACCOUNT_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use solana_sdk::pubkey::Pubkey;
    use crate::accounts::instruments::instruments_data::{INSTR_SIZE, InstrumentsData};

    #[test]
    fn test_instruments_account_offsets_and_size() {
        assert_eq!(mem::size_of::<u64>(), 8);
        assert_eq!(mem::size_of::<u32>(), 4);
        assert_eq!(mem::size_of::<u8>(), 1);
        assert_eq!(mem::size_of::<Pubkey>(), 32);
        assert_eq!(mem::size_of::<InstrumentsData>(), INSTR_SIZE);

        let account = InstrumentsAccount {
            header: AccountHeader{
                version: 0,
                root: Pubkey::from([1; 32]),
                tag: 0,
            },
            count: 0,
            data: [InstrumentsData {
                spot_address: Pubkey::default(),
                mint_id: 0,
                px_decimals: 0,
                strike_decimals: 0,
                asset_ticker: [0; 8],
                assigned_oracle_num: 0,
                reserved: 0,
                oracle_time: 0,
                workers_count: 0,
                workers_data: [0; 128],
            }; INSTR_DATA_COUNT],
        };

        let base_ptr = &account as *const _ as usize;
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, INSTR_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, INSTR_ACCOUNT_COUNT_OFFSET);
        assert_eq!(unsafe { &account.data as *const _ as usize } - base_ptr, INSTR_ACCOUNT_DATA_OFFSET);

        assert_eq!(mem::size_of::<InstrumentsAccount>(), INSTR_ACCOUNT_SIZE);
    }
}
