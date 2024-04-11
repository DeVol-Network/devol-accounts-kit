use std::cell::Ref;
use std::error::Error;
use solana_program::account_info::{Account, AccountInfo, IntoAccountInfo};
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::client_account::client_lp::ClientLp;
use crate::accounts::client::client_account::client_mint::ClientMint;
use crate::accounts::client::client_account::client_pool::ClientPool;
use crate::accounts::client::client_account::client_sign_method::ClientSignMethod;
use crate::accounts::client::client_account::kyc_status::KYCStatus;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_expandable_size_account::DevolExpandableSizeAccount;
use crate::accounts::mints::mints_account::MAX_MINTS_COUNT;
use crate::accounts::worker::pools_log::pool_log::POOLS_LOG_SIZE;
use crate::constants::HOURS;
use crate::dvl_error::DvlError;
use crate::errors::*;
use crate::utils::type_size_helper::align_size;

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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClientAccount {
    pub header: AccountHeader,          // 40 bytes, CLIENT_ACCOUNT_VERSION_OFFSET
    pub owner_address: Pubkey,          // 32 bytes, CLIENT_ACCOUNT_OWNER_ADDRESS_OFFSET
    pub signer_address: Pubkey,         // 32 bytes, CLIENT_ACCOUNT_SIGNER_ADDRESS_OFFSET
    pub payoff_log: Pubkey,             // 32 bytes, CLIENT_ACCOUNT_PAYOFF_LOG_OFFSET
    pub id: u32,                        // 4 bytes, CLIENT_ACCOUNT_ID_OFFSET
    ops_counter: [u8; 8],               // 8 bytes, CLIENT_ACCOUNT_OPS_COUNTER_OFFSET
    pub sign_method: ClientSignMethod,  // 4 bytes, CLIENT_ACCOUNT_SIGN_METHOD_OFFSET
    pub kyc_status: KYCStatus,          // 8 bytes, CLIENT_ACCOUNT_KYC_OFFSET
    pub kyc_time: u32,                  // 4 bytes, CLIENT_ACCOUNT_KYC_TIME_OFFSET
    pub last_day: u32,                  // 4 bytes, CLIENT_ACCOUNT_LAST_DAY_OFFSET
    pub last_hour: u32,                 // 4 bytes, CLIENT_ACCOUNT_LAST_HOUR_OFFSET
    pub last_trades: [u8; 8 * HOURS],   // 192 bytes, CLIENT_ACCOUNT_LAST_TRADES_OFFSET
    refs: [u8; 8],                      // 8 bytes, CLIENT_ACCOUNT_REFS_OFFSET
    pub mints: [ClientMint; MAX_MINTS_COUNT],   // 512 bytes, CLIENT_ACCOUNT_MINTS_OFFSET
    pub lp_count: u32,                  // 4 bytes, CLIENT_ACCOUNT_LP_COUNT_OFFSET
    pub lp: [ClientLp; MAX_CLIENT_LP_COUNT],    // 8192 bytes, CLIENT_ACCOUNT_LP_OFFSET
    pub pools_count: [u8; 4],           // 4 bytes, CLIENT_ACCOUNT_POOLS_COUNT_OFFSET
    /// WARNING!!! Unaligned, wrong address, use getter and setter!
    pools: [ClientPool; 0],             // extendable size, CLIENT_ACCOUNT_POOLS_OFFSET
}

impl DevolExpandableSizeAccount for ClientAccount {
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize {
        let account = unsafe { &*(account_data.as_ptr() as *const Self) };
        let pools_count = account.get_pools_count();
        let expected_size = align_size(CLIENT_ACCOUNT_SIZE + (pools_count as usize) * POOLS_LOG_SIZE, 8);
        expected_size
    }
}
impl DevolAccount for ClientAccount {
    fn expected_size() -> usize {
        CLIENT_ACCOUNT_SIZE
    }

    fn expected_tag() -> u8 {
        CLIENT_ACCOUNT_TAG
    }

    fn expected_version() -> u32 {
        CLIENT_ACCOUNT_VERSION as u32
    }
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
    pub fn get_pool(&self, index: usize) -> Result<&ClientPool, DvlError> {
        if index >= self.get_pools_count() as usize {
            return Err(DvlError::new_with_account(AccountTag::Client, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.pools_count.as_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(CLIENT_ACCOUNT_POOLS_OFFSET - CLIENT_ACCOUNT_POOLS_COUNT_OFFSET) as *const ClientPool
        };
        Ok(unsafe { &*pools_ptr.add(index) })
    }

    #[inline(always)]
    pub fn get_pool_mut(&mut self, index: usize) -> Result<&mut ClientPool, DvlError> {
        if index >= self.get_pools_count() as usize {
            return Err(DvlError::new_with_account(AccountTag::Client, ContractError::PoolRecordNotFound));
        }
        let pools_count_ptr = self.pools_count.as_mut_ptr();
        let pools_ptr = unsafe {
            pools_count_ptr.add(CLIENT_ACCOUNT_POOLS_OFFSET - CLIENT_ACCOUNT_POOLS_COUNT_OFFSET) as *mut ClientPool
        };
        Ok(unsafe { &mut *pools_ptr.add(index) })
    }

    #[inline(always)]
    fn check_signer(
        account: &ClientAccount,
        signer: &Pubkey,
        devol_sign: bool,) -> Result<(), DvlError> {
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        if devol_sign && account.signer_address != *signer {
            Err(DvlError::new_with_account(tag, ContractError::AccountNotSigner))
        } else if !devol_sign && account.owner_address != *signer {
            Err(DvlError::new_with_account(tag, ContractError::AccountNotSigner))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_all(
        account_info: &AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<(), DvlError> {
        Self::check_basic(account_info,root_addr,program_id)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Self::check_signer(account, signer, devol_sign)?;
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        Self::check_expanded_size(tag, account_info.data.borrow())?;
        Ok(())
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    #[inline(always)]
    pub fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<&'a Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, signer, devol_sign)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    #[inline(always)]
    pub fn from_account_info_mut<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<&'a mut Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, signer, devol_sign)?;
        if !account_info.is_writable {
            return Err(DvlError::new_with_account(AccountTag::from_u8(Self::expected_tag()).unwrap(), ContractError::AccountWritableAttribute));
        }
        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    #[inline(always)]
    pub fn from_account(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer: &Pubkey,
        devol_sign: bool,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id, signer, devol_sign)?;
        Ok(Box::new(*account_ref))
    }

    #[allow(dead_code)]
    fn serialize_mut(&mut self) -> &mut [u8] {
        let size = std::mem::size_of::<ClientAccount>();
        unsafe { std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, size) }
    }
}

#[cfg(test)]
impl Default for ClientAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader{
                tag: CLIENT_ACCOUNT_TAG as u32,
                version: CLIENT_ACCOUNT_VERSION as u32,
                root: Pubkey::new_unique(),
            },
            owner_address: Pubkey::new_unique(),
            signer_address: Pubkey::new_unique(),
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
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;
    use std::str::FromStr;
    use crate::utils::type_size_helper::align_size;
    use super::*;
    use solana_program::account_info::{AccountInfo};
    use crate::accounts::worker::pools_log::pool_log::POOLS_LOG_SIZE;
    use crate::constants::test_constants;

    #[test]
    fn test_client_account_offsets() {
        let account = ClientAccount::default();

        let base_ptr = &account as *const _ as usize;

        assert_eq!(&account.header as *const _ as usize - base_ptr, CLIENT_ACCOUNT_VERSION_OFFSET);
        assert_eq!(&account.owner_address as *const _ as usize - base_ptr, CLIENT_ACCOUNT_OWNER_ADDRESS_OFFSET);
        assert_eq!(&account.signer_address as *const _ as usize - base_ptr, CLIENT_ACCOUNT_SIGNER_ADDRESS_OFFSET);
        assert_eq!(&account.payoff_log as *const _ as usize - base_ptr, CLIENT_ACCOUNT_PAYOFF_LOG_OFFSET);
        assert_eq!(&account.id as *const _ as usize - base_ptr, CLIENT_ACCOUNT_ID_OFFSET);
        assert_eq!(&account.ops_counter as *const _ as usize - base_ptr, CLIENT_ACCOUNT_OPS_COUNTER_OFFSET);
        assert_eq!(&account.sign_method as *const _ as usize - base_ptr, CLIENT_ACCOUNT_SIGN_METHOD_OFFSET);
        assert_eq!(&account.kyc_status as *const _ as usize - base_ptr, CLIENT_ACCOUNT_KYC_OFFSET);
        assert_eq!(&account.kyc_time as *const _ as usize - base_ptr, CLIENT_ACCOUNT_KYC_TIME_OFFSET);
        assert_eq!(&account.last_day as *const _ as usize - base_ptr, CLIENT_ACCOUNT_LAST_DAY_OFFSET);
        assert_eq!(&account.last_hour as *const _ as usize - base_ptr, CLIENT_ACCOUNT_LAST_HOUR_OFFSET);
        assert_eq!(&account.last_trades as *const _ as usize - base_ptr, CLIENT_ACCOUNT_LAST_TRADES_OFFSET);
        assert_eq!(&account.refs as *const _ as usize - base_ptr, CLIENT_ACCOUNT_REFS_OFFSET);
        assert_eq!(&account.mints as *const _ as usize - base_ptr, CLIENT_ACCOUNT_MINTS_OFFSET);
        assert_eq!(&account.lp as *const _ as usize - base_ptr, CLIENT_ACCOUNT_LP_OFFSET);
        assert_eq!(&account.pools_count as *const _ as usize - base_ptr, CLIENT_ACCOUNT_POOLS_COUNT_OFFSET);
        // WARNING: This test will not pass because of the alignment. Getter and setter use correct address.
        // assert_eq!(&account.pools as *const _ as usize - base_ptr, CLIENT_ACCOUNT_POOLS_OFFSET);

        assert_eq!(mem::size_of::<ClientAccount>(), align_size(CLIENT_ACCOUNT_SIZE, 8));
    }
    #[test]
    fn test_client_account_pools() {
        let program_id = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let owner = program_id;
        let devol_sign = true;
        let mut lamports: u64 = 0;

        const TEST_POOLS_SIZE: usize = 10;
        let total_size = align_size(CLIENT_ACCOUNT_SIZE + TEST_POOLS_SIZE * POOLS_LOG_SIZE, 4);

        let mut buffer_for_account = vec![0u8; total_size];
        let account = unsafe { &mut *(buffer_for_account.as_mut_ptr() as *mut ClientAccount) };
        *account = ClientAccount::default();
        account.set_pools_count(TEST_POOLS_SIZE as u32);
        let check_1 = (0, 228);
        let check_2 = (7, 69);
        account.get_pool_mut(check_1.0).unwrap().fractions = check_1.1;
        account.get_pool_mut(check_2.0).unwrap().instr_id = check_2.1;

        let key = account.owner_address;
        let signer = account.signer_address;
        let root = account.header.root;

        let account_info = AccountInfo{
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(buffer_for_account.as_mut_slice())),
            owner: &owner,
            rent_epoch: 0,
            is_signer: devol_sign,
            is_writable: false,
            executable: false,
        };

        let new_account = ClientAccount::from_account_info(&account_info, &root,
                                                           &program_id, &signer, devol_sign).unwrap();

        assert_eq!(new_account.get_pool(check_1.0).unwrap().fractions, check_1.1);
        assert_eq!(new_account.get_pool(check_2.0).unwrap().instr_id, check_2.1);
    }
    #[test]
    fn test_client_account_check_no_signer() {
        let owner = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let key = Pubkey::from_str("CTVKkHWP7AF8KuLzmxcJevpNj9YaocbxkaN5QwnhtSPm").unwrap();
        let root = Pubkey::from_str(test_constants::ROOT_ADDRESS).unwrap();
        let program_id = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let signer = &key;
        let is_signer = false;
        let mut lamports: u64 = 0;

        let mut account = ClientAccount {
            header: AccountHeader{
                tag: CLIENT_ACCOUNT_TAG as u32,
                version: CLIENT_ACCOUNT_VERSION as u32,
                root,
            },
            owner_address: key,
            signer_address: *signer,
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
        };

        let serialized_data = account.serialize_mut();

        let account_info = AccountInfo{
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(serialized_data)),
            owner: &owner,
            rent_epoch: 0,
            is_signer,
            is_writable: false,
            executable: false,
        };
        let result = ClientAccount::check_all(&account_info, &root, &program_id, signer, is_signer);
        assert!(result.is_ok());
    }
    #[test]
    fn test_client_account_check_with_signer() {
        let owner = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let key = Pubkey::from_str("CTVKkHWP7AF8KuLzmxcJevpNj9YaocbxkaN5QwnhtSPm").unwrap();
        let root = Pubkey::from_str(test_constants::ROOT_ADDRESS).unwrap();
        let program_id = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let signer = Pubkey::from_str("123KkHWP7AF8KuLzmxcJevpNj9YaocbxkaN5QwnhtSPm").unwrap();
        let devol_sign = true;
        let mut lamports: u64 = 0;

        let mut account = ClientAccount {
            header: AccountHeader{
                tag: CLIENT_ACCOUNT_TAG as u32,
                version: CLIENT_ACCOUNT_VERSION as u32,
                root,
            },
            owner_address: key,
            signer_address: signer,
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
        };

        let serialized_data = account.serialize_mut();

        let account_info = AccountInfo{
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(serialized_data)),
            owner: &owner,
            rent_epoch: 0,
            is_signer: devol_sign,
            is_writable: false,
            executable: false,
        };
        let result = ClientAccount::check_all(&account_info, &root, &program_id, &signer, devol_sign);
        assert!(result.is_ok());
    }
}

