use solana_program::pubkey::Pubkey;

pub const INSTR_SPOT_ADDRESS_OFFSET: usize = 0;
pub const INSTR_MINT_ID_OFFSET: usize = 32;
pub const INSTR_PX_DECIMALS_OFFSET: usize = 36;
pub const INSTR_STRIKE_DECIMALS_OFFSET: usize = 40;
pub const INSTR_ASSET_TICKER_OFFSET: usize = 44;
pub const INSTR_ASSIGNED_ORACLE_NUM_OFFSET: usize = 52;
pub const INSTR_RESERVED_OFFSET: usize = 56;
pub const INSTR_ORACLE_TIME_OFFSET: usize = 60;
pub const INSTR_WORKERS_COUNT_OFFSET: usize = 64;
pub const INSTR_WORKERS_DATA_OFFSET: usize = 68;
pub const INSTR_SIZE: usize = 196;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstrumentsData {
    pub spot_address: Pubkey,                // 32 bytes, INSTR_SPOT_ADDRESS_OFFSET
    pub mint_id: u32,                        //  4 bytes, INSTR_MINT_ID_OFFSET
    pub px_decimals: u32,                    //  4 bytes, INSTR_PX_DECIMALS_OFFSET
    pub strike_decimals: u32,                //  4 bytes, INSTR_STRIKE_DECIMALS_OFFSET
    pub asset_ticker: [u8; 8],               //  8 bytes, INSTR_ASSET_TICKER_OFFSET
    pub assigned_oracle_num: i32,            //  4 bytes, INSTR_ASSIGNED_ORACLE_NUM_OFFSET
    pub reserved: u32,                       //  4 bytes, INSTR_RESERVED_OFFSET
    pub oracle_time: i32,                    //  4 bytes, INSTR_ORACLE_TIME_OFFSET
    pub workers_count: u32,                  //  4 bytes, INSTR_WORKERS_COUNT_OFFSET
    pub workers_data: [u8; 128],             //128 bytes, INSTR_WORKERS_DATA_OFFSET
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_instruments_account_offsets() {
        assert_eq!(mem::size_of::<u64>(), 8);
        assert_eq!(mem::size_of::<u32>(), 4);
        assert_eq!(mem::size_of::<u8>(), 1);
        assert_eq!(mem::size_of::<Pubkey>(), 32);

        let account = InstrumentsData {
            spot_address: Pubkey::from([1; 32]),
            mint_id: 0,
            px_decimals: 0,
            strike_decimals: 0,
            asset_ticker: [0; 8],
            assigned_oracle_num: 0,
            reserved: 0,
            oracle_time: 0,
            workers_count: 0,
            workers_data: [0; 128],
        };

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(unsafe { &account.spot_address as *const _ as usize } - base_ptr, INSTR_SPOT_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.mint_id as *const _ as usize } - base_ptr, INSTR_MINT_ID_OFFSET);
        assert_eq!(unsafe { &account.px_decimals as *const _ as usize } - base_ptr, INSTR_PX_DECIMALS_OFFSET);
        assert_eq!(unsafe { &account.strike_decimals as *const _ as usize } - base_ptr, INSTR_STRIKE_DECIMALS_OFFSET);
        assert_eq!(unsafe { &account.asset_ticker as *const _ as usize } - base_ptr, INSTR_ASSET_TICKER_OFFSET);
        assert_eq!(unsafe { &account.assigned_oracle_num as *const _ as usize } - base_ptr, INSTR_ASSIGNED_ORACLE_NUM_OFFSET);
        assert_eq!(unsafe { &account.reserved as *const _ as usize } - base_ptr, INSTR_RESERVED_OFFSET);
        assert_eq!(unsafe { &account.oracle_time as *const _ as usize } - base_ptr, INSTR_ORACLE_TIME_OFFSET);
        assert_eq!(unsafe { &account.workers_count as *const _ as usize } - base_ptr, INSTR_WORKERS_COUNT_OFFSET);
        assert_eq!(unsafe { &account.workers_data as *const _ as usize } - base_ptr, INSTR_WORKERS_DATA_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<InstrumentsData>(), INSTR_SIZE);
    }
}
