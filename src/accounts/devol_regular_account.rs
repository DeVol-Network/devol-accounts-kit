use std::error::Error;
use solana_program::account_info::{Account, AccountInfo, IntoAccountInfo};
use solana_program::pubkey::Pubkey;
use crate::accounts::devol_account::DevolAccount;
use crate::errors::{ReadAccountError};

pub trait DevolRegularAccount: DevolAccount {
    #[inline(always)]
    fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<(), u32> {
        Self::check_basic(account_info, root_addr, program_id)
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<&'a Self, u32>
        where
            Self: Sized,
    {
        Self::from_account_info_basic(account_info, root_addr, program_id)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    fn from_account_info_mut<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<&'a mut Self, u32>
        where
            Self: Sized,
    {
        Self::from_account_info_mut_basic(account_info, root_addr, program_id)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    fn from_account(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id)
            .map_err(ReadAccountError::from)?;
        Ok(Box::new(*account_ref))
    }
}