use std::error::Error;
use solana_program::account_info::{Account, AccountInfo, IntoAccountInfo};
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::client_account::client_mint::ClientMint;
use crate::accounts::client::client_account::common::client_sign_method::ClientSignMethod;
use crate::accounts::client::client_account::common::kyc_status::KYCStatus;
use crate::accounts::client::client_account::common::signer_account_params::SignerAccountParams;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::mints::mints_account::MAX_MINTS_COUNT;
use crate::constants::HOURS_IN_DAY;
use crate::dvl_error::DvlError;
use crate::errors::*;

pub const CLIENT_ACCOUNT_SIZE: usize = 9084;
pub const CLIENT_ACCOUNT_TAG: u8 = 8;
pub const CLIENT_ACCOUNT_VERSION: usize = 10;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClientAccount {
    pub header: AccountHeader,                   // 40 bytes
    pub owner_address: Pubkey,                   // 32 bytes
    pub signer_address: Pubkey,                  // 32 bytes
    pub payoff_log_address: Pubkey,              // 32 bytes
    pub lp_portfolio_address: Pubkey,            // 32 bytes
    pub portfolio_address: Pubkey,               // 32 bytes
    pub id: u32,                                 // 4 bytes (4/8 bytes align)
    pub sign_method: ClientSignMethod,           // 4 bytes (8/8 bytes align)
    pub kyc_status: KYCStatus,                   // 8 bytes
    pub kyc_time: u32,                           // 4 bytes (4/8 bytes align)
    pub last_trade_hour_since_epoch: u32,        // 4 bytes (8/8 bytes align)
    pub hours_trade_volume: [u64; HOURS_IN_DAY], // 192 bytes
    pub mints: [ClientMint; MAX_MINTS_COUNT],    // 512 bytes
}

impl DevolAccount for ClientAccount {
    #[inline(always)]
    fn expected_size() -> usize {
        CLIENT_ACCOUNT_SIZE
    }

    #[inline(always)]
    fn expected_tag() -> u8 {
        CLIENT_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 {
        CLIENT_ACCOUNT_VERSION as u32
    }
}

impl ClientAccount {

    #[inline(always)]
    fn check_signer(
        account: &ClientAccount,
        signer_params: Option<&SignerAccountParams>,
    ) -> Result<(), DvlError> {
        if let Some(signer_params) = signer_params{
            let tag = AccountTag::from_u8(Self::expected_tag());
            if signer_params.devol_sign && account.signer_address != *signer_params.signer {
                return Err(DvlError::new_with_account(tag, ContractError::AccountNotSigner))
            } else if !signer_params.devol_sign && account.owner_address != *signer_params.signer {
                return Err(DvlError::new_with_account(tag, ContractError::AccountNotSigner))
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn check_all(
        account_info: &AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer_params: Option<&SignerAccountParams>,
    ) -> Result<(), DvlError> {
        Self::check_basic(account_info,root_addr,program_id)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Self::check_signer(account, signer_params)?;
        Ok(())
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    #[inline(always)]
    pub fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        signer_params: Option<&SignerAccountParams>,
    ) -> Result<&'a Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, signer_params)?;
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
        signer_params: Option<&SignerAccountParams>,
    ) -> Result<&'a mut Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, signer_params)?;
        if !account_info.is_writable {
            return Err(DvlError::new_with_account(AccountTag::from_u8(Self::expected_tag()), ContractError::AccountWritableAttribute));
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
        signer_params: Option<&SignerAccountParams>,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id, signer_params)?;
        Ok(Box::new(*account_ref))
    }

    #[cfg(test)]
    fn serialize_mut(&mut self) -> &mut [u8] {
        let size = std::mem::size_of::<ClientAccount>();
        unsafe { std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, size) }
    }
}

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
            payoff_log_address: Pubkey::new_unique(),
            lp_portfolio_address: Pubkey::new_unique(),
            portfolio_address: Pubkey::new_unique(),
            id: 0,
            sign_method: ClientSignMethod::Wallet,
            kyc_status: KYCStatus::Light,
            kyc_time: 0,
            last_trade_hour_since_epoch: 0,
            hours_trade_volume: [0; HOURS_IN_DAY],
            mints: [ClientMint::default(); MAX_MINTS_COUNT],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;
    use std::str::FromStr;
    use super::*;
    use solana_program::account_info::{AccountInfo};
    use crate::constants::test_constants;

    #[test]
    fn test_client_account_offsets() {
        assert_eq!(mem::size_of::<ClientAccount>(), CLIENT_ACCOUNT_SIZE);
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
            payoff_log_address: Pubkey::default(),
            lp_portfolio_address: Pubkey::default(),
            portfolio_address: Pubkey::default(),
            id: 0,
            sign_method: ClientSignMethod::Wallet,
            kyc_status: KYCStatus::Light,
            kyc_time: 0,
            last_trade_hour_since_epoch: 0,
            hours_trade_volume: [0; HOURS_IN_DAY],
            mints: [ClientMint::default(); MAX_MINTS_COUNT],
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
        let result = ClientAccount::check_all(&account_info, &root, &program_id, Some(&SignerAccountParams{signer, devol_sign: is_signer}));
        assert!(result.is_ok());
    }
    #[test]
    fn test_client_account_check_with_signer() {
        let owner = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let key = Pubkey::from_str("CTVKkHWP7AF8KuLzmxcJevpNj9YaocbxkaN5QwnhtSPm").unwrap();
        let root = Pubkey::from_str(test_constants::ROOT_ADDRESS).unwrap();
        let program_id = Pubkey::from_str(test_constants::PROGRAM_ID).unwrap();
        let signer = &Pubkey::from_str("123KkHWP7AF8KuLzmxcJevpNj9YaocbxkaN5QwnhtSPm").unwrap();
        let devol_sign = true;
        let mut lamports: u64 = 0;

        let mut account = ClientAccount {
            header: AccountHeader{
                tag: CLIENT_ACCOUNT_TAG as u32,
                version: CLIENT_ACCOUNT_VERSION as u32,
                root,
            },
            owner_address: key,
            signer_address: signer.clone(),
            payoff_log_address: Pubkey::default(),
            lp_portfolio_address: Pubkey::default(),
            portfolio_address: Pubkey::default(),
            id: 0,
            sign_method: ClientSignMethod::Wallet,
            kyc_status: KYCStatus::Light,
            kyc_time: 0,
            last_trade_hour_since_epoch: 0,
            hours_trade_volume: [0; HOURS_IN_DAY],
            mints: [ClientMint::default(); MAX_MINTS_COUNT],
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
        let result = ClientAccount::check_all(&account_info, &root, &program_id, Some(&SignerAccountParams{signer, devol_sign}));
        assert!(result.is_ok());
    }
}

