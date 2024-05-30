use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::client::trade_log::trade_log::TradeLog;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;

pub const TRADE_LOG_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const TRADE_LOG_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const TRADE_LOG_ACCOUNT_MAIN_ADDRESS_OFFSET: usize = 40;
pub const TRADE_LOG_ACCOUNT_LAST_OFFSET: usize = 72;
pub const TRADE_LOG_ACCOUNT_COUNT_OFFSET: usize = 76;
pub const TRADE_LOG_ACCOUNT_DATA_OFFSET: usize = 80;
pub const TRADE_LOG_ACCOUNT_SIZE: usize = 40816;
pub const TRADE_LOG_ACCOUNT_TAG: u8 = 12;
pub const TRADE_LOG_ACCOUNT_VERSION: u32 = 14;
pub const TRADE_LOG_BUFFER_CAPACITY: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TradeLogAccount {
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    // 32 bytes, TRADE_LOG_ACCOUNT_MAIN_ADDRESS_OFFSET
    pub main_address: Pubkey,
    // 4 bytes, TRADE_LOG_ACCOUNT_LAST_OFFSET
    pub last: u32,
    // 4 bytes, TRADE_LOG_ACCOUNT_COUNT_OFFSET
    pub count: u32,
    // 40736 bytes, TRADE_LOG_ACCOUNT_DATA_OFFSET
    pub data: [TradeLog; TRADE_LOG_BUFFER_CAPACITY],
}
impl DevolIndexedAccount for TradeLogAccount {}

impl DevolAccount for TradeLogAccount {
    #[inline(always)]
    fn expected_size() -> usize { TRADE_LOG_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 {
        TRADE_LOG_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 {
        TRADE_LOG_ACCOUNT_VERSION
    }
}

#[cfg(test)]
impl Default for TradeLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            main_address: Pubkey::default(),
            last: 0,
            count: 0,
            data: [TradeLog::default(); TRADE_LOG_BUFFER_CAPACITY],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_log_account_offsets() {
        let account = TradeLogAccount::default();
        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &account.header as *const _ as usize - base_ptr,
            0
        );
        assert_eq!(
            &account.main_address as *const _ as usize - base_ptr,
            TRADE_LOG_ACCOUNT_MAIN_ADDRESS_OFFSET
        );
        assert_eq!(
            &account.last as *const _ as usize - base_ptr,
            TRADE_LOG_ACCOUNT_LAST_OFFSET
        );
        assert_eq!(
            &account.count as *const _ as usize - base_ptr,
            TRADE_LOG_ACCOUNT_COUNT_OFFSET
        );
        assert_eq!(
            &account.data as *const _ as usize - base_ptr,
            TRADE_LOG_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TradeLogAccount>(), TRADE_LOG_ACCOUNT_SIZE);
    }
}
