use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::client_account::client_lp::ClientLp;
use crate::accounts::client::client_account::client_mint::ClientMint;
use crate::accounts::client::client_account::client_pool::ClientPool;
use crate::accounts::client::client_account::client_sign_method::ClientSignMethod;
use crate::accounts::client::client_account::kyc_status::KYCStatus;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mints_account::MAX_MINTS_COUNT;
use crate::constants::HOURS;
use crate::errors::*;

// Проверить что гет пулс работает (расширение и чтение, запись и чтение, мут/не мут)

pub const CLIENT_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const CLIENT_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const CLIENT_ACCOUNT_OWNER_ADDRESS_OFFSET: usize = 40;
pub const CLIENT_ACCOUNT_SIGNER_ADDRESS_OFFSET: usize = 72;
pub const CLIENT_ACCOUNT_PAYOFF_LOG_OFFSET: usize = 104;
pub const CLIENT_ACCOUNT_ID_OFFSET: usize = 136;
pub const CLIENT_ACCOUNT_OPS_COUNTER_OFFSET: usize = 140;
pub const CLIENT_ACCOUNT_SIGN_METHOD_OFFSET: usize = 148;
pub const CLIENT_ACCOUNT_KYC_OFFSET: usize = 152;
pub const CLIENT_ACCOUNT_KYC_TIME_OFFSET: usize = 160;
pub const CLIENT_ACCOUNT_LAST_DAY_OFFSET: usize = 164;
pub const CLIENT_ACCOUNT_LAST_HOUR_OFFSET: usize = 168;
pub const CLIENT_ACCOUNT_LAST_TRADES_OFFSET: usize = 172;
pub const CLIENT_ACCOUNT_REFS_OFFSET: usize = 364;
pub const CLIENT_ACCOUNT_MINTS_OFFSET: usize = 372;
pub const CLIENT_ACCOUNT_LP_COUNT_OFFSET: usize = 884;
pub const CLIENT_ACCOUNT_LP_OFFSET: usize = 888;
pub const CLIENT_ACCOUNT_POOLS_COUNT_OFFSET: usize = 9080;
pub const CLIENT_ACCOUNT_POOLS_OFFSET: usize = 9084;
pub const CLIENT_ACCOUNT_SIZE: usize = 9084;
pub const CLIENT_ACCOUNT_TAG: u8 = 8;
pub const CLIENT_ACCOUNT_VERSION: usize = 10;
pub const MAX_CLIENT_LP_COUNT: usize = 128;
pub const MAX_CLIENT_POOLS_COUNT: usize = 256;

#[repr(C)]
pub struct ClientAccount {
    pub header: AccountHeader,
    // 40 bytes, CLIENT_ACCOUNT_VERSION_OFFSET
    pub owner_address: Pubkey,
    // 32 bytes, CLIENT_ACCOUNT_OWNER_ADDRESS_OFFSET
    pub signer_address: Pubkey,
    // 32 bytes, CLIENT_ACCOUNT_SIGNER_ADDRESS_OFFSET
    pub payoff_log: Pubkey,
    // 32 bytes, CLIENT_ACCOUNT_PAYOFF_LOG_OFFSET
    pub id: u32,
    // 4 bytes, CLIENT_ACCOUNT_ID_OFFSET
    ops_counter: [u8; 8],
    // 8 bytes, CLIENT_ACCOUNT_OPS_COUNTER_OFFSET
    pub sign_method: ClientSignMethod,
    // 4 bytes, CLIENT_ACCOUNT_SIGN_METHOD_OFFSET
    pub kyc_status: KYCStatus,
    // 8 bytes, CLIENT_ACCOUNT_KYC_OFFSET
    pub kyc_time: u32,
    // 4 bytes, CLIENT_ACCOUNT_KYC_TIME_OFFSET
    pub last_day: u32,
    // 4 bytes, CLIENT_ACCOUNT_LAST_DAY_OFFSET
    pub last_hour: u32,
    // 4 bytes, CLIENT_ACCOUNT_LAST_HOUR_OFFSET
    pub last_trades: [u8; 8 * HOURS],
    // 192 bytes, CLIENT_ACCOUNT_LAST_TRADES_OFFSET
    refs: [u8; 8],
    // 8 bytes, CLIENT_ACCOUNT_REFS_OFFSET
    pub mints: [ClientMint; MAX_MINTS_COUNT],
    // 512 bytes, CLIENT_ACCOUNT_MINTS_OFFSET
    pub lp_count: u32,
    // 4 bytes, CLIENT_ACCOUNT_LP_COUNT_OFFSET
    pub lp: [ClientLp; MAX_CLIENT_LP_COUNT],
    // 8192 bytes, CLIENT_ACCOUNT_LP_OFFSET
    pub pools_count: [u8; 4],
    // 4 bytes, CLIENT_ACCOUNT_POOLS_COUNT_OFFSET
    /// WARNING!!! Unaligned, wrong address, use getter and setter!
    pools: [ClientPool; 0],                     // extendable size, CLIENT_ACCOUNT_POOLS_OFFSET
}

impl DevolAccount for ClientAccount {
    #[inline(always)]
    fn expected_size() -> usize { 0 }

    #[inline(always)]
    fn expected_tag() -> u8 { CLIENT_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { CLIENT_ACCOUNT_VERSION as u32 }

    #[inline(always)]
    fn id_offset_if_available() -> usize { CLIENT_ACCOUNT_ID_OFFSET }
}

impl ClientAccount {
    #[inline(always)]
    pub fn get_ops_counter(&self) -> i64 { i64::from_ne_bytes(self.ops_counter) }

    #[inline(always)]
    pub fn set_ops_counter(&mut self, value: i64) { self.ops_counter = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_refs(&self) -> i64 { i64::from_ne_bytes(self.refs) }

    #[inline(always)]
    pub fn set_refs(&mut self, value: i64) { self.refs = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_pools_count(&self) -> u32 { u32::from_ne_bytes(self.pools_count) }

    #[inline(always)]
    pub fn set_pools_count(&mut self, value: u32) { self.pools_count = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_pool(&self, index: usize) -> Result<&ClientPool, u32> {
        if index >= self.get_pools_count() as usize {
            return Err(error_with_account(AccountTag::Client, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.pools_count.as_ptr();
        let pools_ptr = unsafe { pools_count_ptr.add(CLIENT_ACCOUNT_POOLS_OFFSET - CLIENT_ACCOUNT_POOLS_COUNT_OFFSET) as *const ClientPool };
        Ok(unsafe { &*pools_ptr.add(index) })
    }

    #[inline(always)]
    pub fn get_pool_mut(&mut self, index: usize) -> Result<&mut ClientPool, u32> {
        if index >= self.get_pools_count() as usize {
            return Err(error_with_account(AccountTag::Client, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.pools_count.as_mut_ptr();
        let pools_ptr = unsafe { pools_count_ptr.add(CLIENT_ACCOUNT_POOLS_OFFSET - CLIENT_ACCOUNT_POOLS_COUNT_OFFSET) as *mut ClientPool};
        Ok(unsafe { &mut *pools_ptr.add(index) })
    }
}

#[cfg(test)]
impl Default for ClientAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            owner_address: Pubkey::default(),
            signer_address: Pubkey::default(),
            payoff_log: Pubkey::default(),
            id: 0,
            ops_counter: [0; 8],
            sign_method: ClientSignMethod::Wallet,
            kyc_status: KYCStatus::Light,
            kyc_time: 0,
            last_day: 0,
            last_hour: 0,
            last_trades: [0; 8 * HOURS],
            refs: [0; 8],
            mints: [ClientMint::default(); MAX_MINTS_COUNT],
            lp_count: 0,
            lp: [ClientLp::default(); MAX_CLIENT_LP_COUNT],
            pools_count: [0; 4],
            pools: [],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_client_account_offsets() {
        let account = ClientAccount::default();

        let base_ptr = &account as *const _ as usize;

        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.owner_address as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_OWNER_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.signer_address as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_SIGNER_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.payoff_log as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_PAYOFF_LOG_OFFSET);
        assert_eq!(unsafe { &account.id as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_ID_OFFSET);
        assert_eq!(unsafe { &account.ops_counter as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_OPS_COUNTER_OFFSET);
        assert_eq!(unsafe { &account.sign_method as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_SIGN_METHOD_OFFSET);
        assert_eq!(unsafe { &account.kyc_status as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_KYC_OFFSET);
        assert_eq!(unsafe { &account.kyc_time as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_KYC_TIME_OFFSET);
        assert_eq!(unsafe { &account.last_day as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_LAST_DAY_OFFSET);
        assert_eq!(unsafe { &account.last_hour as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_LAST_HOUR_OFFSET);
        assert_eq!(unsafe { &account.last_trades as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_LAST_TRADES_OFFSET);
        assert_eq!(unsafe { &account.refs as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_REFS_OFFSET);
        assert_eq!(unsafe { &account.mints as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_MINTS_OFFSET);
        assert_eq!(unsafe { &account.lp as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_LP_OFFSET);
        assert_eq!(unsafe { &account.pools_count as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_POOLS_COUNT_OFFSET);
        // WARNING: This test will not pass because of the alignment:
        // assert_eq!(unsafe { &account.pools as *const _ as usize } - base_ptr, CLIENT_ACCOUNT_POOLS_OFFSET);

        assert_eq!(mem::size_of::<ClientAccount>(), align_size(CLIENT_ACCOUNT_SIZE, 8));
    }
}

