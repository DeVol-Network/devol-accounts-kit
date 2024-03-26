use solana_sdk::account_info::AccountInfo;
use crate::accounts::devol_account::DevolAccount;

// #[inline(always)]
// pub fn check_account<T: DevolAccount>(account_data: &[u8]) -> Result<(), u32> {
//     T::check_size(account_data.len())?;
//     Ok(())
// }
//
// #[inline(always)]
// pub fn transform_account_info<'a, T: DevolAccount>(account_info: &AccountInfo) -> Result<&'a T, u32> {
//     let account = unsafe { & *(account_info.data.borrow().as_ptr() as *const T) };
//     check_account(account)?;
//     Ok(account)
// }
//
// #[inline(always)]
// pub fn transform_account_data<'a, T: DevolAccount>(account_data: &[u8]) -> Result<&'a T, u32> {
//     let account = unsafe { & *(account_data.as_ptr() as *const T) };
//     check_account(account)?;
//     Ok(account)
// }
//
// #[inline(always)]
// pub fn transform_account_info_mut<'a, T: DevolAccount>(account_info: &AccountInfo) -> Result<&'a mut T, u32> {
//     let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut T) };
//     check_account(account)?;
//     Ok(account)
// }