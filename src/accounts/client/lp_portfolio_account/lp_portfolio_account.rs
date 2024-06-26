use std::cell::Ref;
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::lp_portfolio_account::lp_portfolio_pool_record::{LP_PORTFOLIO_RECORD_SIZE, LpPortfolioPoolRecord};
use crate::accounts::client::portfolio_account::portfolio_account::{PORTFOLIO_ACCOUNT_TAG, PORTFOLIO_ACCOUNT_VERSION};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_expandable_size_account::DevolExpandableSizeAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::dvl_error::DvlError;
use crate::errors::{AccountTag, ContractError};

pub const LP_PORTFOLIO_ACCOUNT_SIZE: usize = 80;
pub const LP_PORTFOLIO_ACCOUNT_TAG: u8 = 14;
pub const LP_PORTFOLIO_ACCOUNT_VERSION: usize = 1;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LpPortfolioAccount {
    pub header: AccountHeader,          // 40 bytes
    pub owner_address: Pubkey,          // 32 bytes
    pub portfolio_records_count: u32,   // 4 bytes (4/8 bytes align)
    reserved: u32,                      // 4 bytes (8/8 bytes align)
    pub portfolio_records: [LpPortfolioPoolRecord; 0], // 64xExpandable bytes
}

impl DevolAccount for LpPortfolioAccount {
    #[inline(always)]
    fn expected_size() -> usize {
        LP_PORTFOLIO_ACCOUNT_SIZE
    }

    #[inline(always)]
    fn expected_tag() -> u8 {
        LP_PORTFOLIO_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 { LP_PORTFOLIO_ACCOUNT_VERSION as u32 }
}

impl DevolRegularAccount for LpPortfolioAccount {}

impl LpPortfolioAccount {
    #[inline(always)]
    pub fn get_portfolio_record(&self, index: usize) -> Result<&LpPortfolioPoolRecord, DvlError> {
        if index >= self.portfolio_records_count as usize {
            return Err(DvlError::new_with_account(AccountTag::ClientLpPortfolio, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.portfolio_records.as_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(index)
        };
        Ok(unsafe { &*pools_ptr })
    }

    #[inline(always)]
    pub fn get_portfolio_record_mut(&mut self, index: usize) -> Result<&mut LpPortfolioPoolRecord, DvlError> {
        if index >= self.portfolio_records_count as usize {
            return Err(DvlError::new_with_account(AccountTag::ClientLpPortfolio, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.portfolio_records.as_mut_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(index)
        };
        Ok(unsafe { &mut *pools_ptr })
    }
}

impl DevolExpandableSizeAccount for LpPortfolioAccount {
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize {
        let account = unsafe { &*(account_data.as_ptr() as *const Self) };
        let pools_count = account.portfolio_records_count;
        let expected_size = LP_PORTFOLIO_ACCOUNT_SIZE + (pools_count as usize) * LP_PORTFOLIO_RECORD_SIZE;
        expected_size
    }
}

impl Default for LpPortfolioAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader{
                tag: LP_PORTFOLIO_ACCOUNT_TAG as u32,
                version: LP_PORTFOLIO_ACCOUNT_VERSION as u32,
                root: Pubkey::new_unique(),
            },
            owner_address: Pubkey::default(),
            reserved: 0,
            portfolio_records_count: 0,
            portfolio_records: [],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::client::lp_portfolio_account::lp_portfolio_account::{LP_PORTFOLIO_ACCOUNT_SIZE, LpPortfolioAccount};

    #[test]
    fn test_client_lp_offsets() {
        assert_eq!(std::mem::size_of::<LpPortfolioAccount>(), LP_PORTFOLIO_ACCOUNT_SIZE);
    }
}
