use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::accounts::devol_account::DevolAccount;

// pub trait DevolSignableAccount: DevolAccount {
//     #[inline(always)]
//     fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<(), u32> {
//         Self::check_basic(account_info, root_addr, program_id)
//     }
// }