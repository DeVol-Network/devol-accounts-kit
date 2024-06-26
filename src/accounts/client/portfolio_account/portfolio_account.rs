use std::cell::Ref;
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::client_account::client_account::{CLIENT_ACCOUNT_TAG, CLIENT_ACCOUNT_VERSION};
use crate::accounts::client::portfolio_account::portfolio_pool_record::{PORTFOLIO_POOL_RECORD_SIZE, PortfolioPoolRecord};
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_expandable_size_account::DevolExpandableSizeAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::dvl_error::DvlError;
use crate::errors::{AccountTag, ContractError};

pub const PORTFOLIO_ACCOUNT_SIZE: usize = 80;
pub const PORTFOLIO_ACCOUNT_TAG: u8 = 15;
pub const PORTFOLIO_ACCOUNT_VERSION: usize = 1;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PortfolioAccount {
    pub header: AccountHeader,          // 40 bytes
    pub owner_address: Pubkey,          // 32 bytes
    pub reserved: u32,                  // 4 bytes (4/8 bytes align)
    pub portfolio_records_count: u32,     // 4 bytes (8/8 bytes align)
    portfolio_records: [PortfolioPoolRecord; 0],    // 8792xExpandable bytes
}

impl DevolAccount for PortfolioAccount {
    #[inline(always)]
    fn expected_size() -> usize {
        PORTFOLIO_ACCOUNT_SIZE
    }

    #[inline(always)]
    fn expected_tag() -> u8 {
        PORTFOLIO_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 { PORTFOLIO_ACCOUNT_VERSION as u32 }
}

impl DevolRegularAccount for PortfolioAccount {}

impl PortfolioAccount {
    #[inline(always)]
    pub fn get_portfolio_record(&self, index: usize) -> Result<&PortfolioPoolRecord, DvlError> {
        if index >= self.portfolio_records_count as usize {
            return Err(DvlError::new_with_account(AccountTag::ClientPortfolio, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.portfolio_records.as_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(index)
        };
        Ok(unsafe { &*pools_ptr })
    }

    #[inline(always)]
    pub fn get_portfolio_record_mut(&mut self, index: usize) -> Result<&mut PortfolioPoolRecord, DvlError> {
        if index >= self.portfolio_records_count as usize {
            return Err(DvlError::new_with_account(AccountTag::ClientPortfolio, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.portfolio_records.as_mut_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(index)
        };
        Ok(unsafe { &mut *pools_ptr })
    }
}

impl DevolExpandableSizeAccount for PortfolioAccount {
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize {
        let account = unsafe { &*(account_data.as_ptr() as *const Self) };
        let pools_count = account.portfolio_records_count;
        let expected_size = PORTFOLIO_ACCOUNT_SIZE + (pools_count as usize) * PORTFOLIO_POOL_RECORD_SIZE;
        expected_size
    }
}

impl Default for PortfolioAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader{
                tag: PORTFOLIO_ACCOUNT_TAG as u32,
                version: PORTFOLIO_ACCOUNT_VERSION as u32,
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::str::FromStr;
    use solana_program::account_info::AccountInfo;
    use solana_program::pubkey::Pubkey;
    use crate::accounts::client::portfolio_account::portfolio_account::{PORTFOLIO_ACCOUNT_SIZE, PortfolioAccount};
    use crate::accounts::client::portfolio_account::portfolio_pool_record::PORTFOLIO_POOL_RECORD_SIZE;
    use crate::accounts::devol_regular_account::DevolRegularAccount;
    use crate::constants::test_constants;

    #[test]
    fn test_portfolio_records() {
        let program_id = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let owner = program_id;
        let devol_sign = true;
        let mut lamports: u64 = 0;

        const TEST_RECORDS_SIZE: usize = 10;
        let total_size = PORTFOLIO_ACCOUNT_SIZE + TEST_RECORDS_SIZE * PORTFOLIO_POOL_RECORD_SIZE;

        let mut buffer_for_account = vec![0u8; total_size];
        let account = unsafe { &mut *(buffer_for_account.as_mut_ptr() as *mut PortfolioAccount) };
        let mut default_account = PortfolioAccount::default();
        default_account.portfolio_records_count = TEST_RECORDS_SIZE as u32;
        *account = default_account;
        let check_1 = (0, 228);
        let check_2 = (7, 69);
        let check_3 = (0, 1337);
        let check_4 = (3, 200);
        let check_4_bucket = 50;
        {
            account.get_portfolio_record_mut(check_1.0).unwrap().fractions = check_1.1;
            account.get_portfolio_record_mut(check_3.0).unwrap().instrument_id = check_3.1;
            account.get_portfolio_record_mut(check_2.0).unwrap().instrument_id = check_2.1;
            account.get_portfolio_record_mut(check_4.0).unwrap().calls_result_pnl[check_4_bucket] = check_4.1;
        }
        let key = &account.owner_address;
        let root = &account.header.root;

        let account_info = AccountInfo {
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(buffer_for_account.as_mut_slice())),
            owner: &owner,
            rent_epoch: 0,
            is_signer: devol_sign,
            is_writable: false,
            executable: false,
        };

        let new_account = PortfolioAccount::from_account_info(&account_info, &root, &program_id).unwrap();

        assert_eq!(new_account.header, default_account.header);
        assert_eq!(new_account.portfolio_records_count, default_account.portfolio_records_count);
        assert_eq!(new_account.owner_address, default_account.owner_address);

        assert_eq!(check_1.1, new_account.get_portfolio_record(check_1.0).unwrap().fractions);
        assert_eq!(check_2.1, new_account.get_portfolio_record(check_2.0).unwrap().instrument_id);
        assert_eq!(check_3.1, new_account.get_portfolio_record(check_3.0).unwrap().instrument_id);
        assert_eq!(check_4.1, new_account.get_portfolio_record(check_4.0).unwrap().calls_result_pnl[check_4_bucket]);
    }
    #[test]
    fn test_client_offsets() {
        assert_eq!(std::mem::size_of::<PortfolioAccount>(), PORTFOLIO_ACCOUNT_SIZE);
    }
}