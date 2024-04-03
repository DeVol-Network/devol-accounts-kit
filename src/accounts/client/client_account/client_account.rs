use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;

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

#[repr(C)]
struct ClientAccount {
    pub header: AccountHeader, // CLIENT_ACCOUNT_VERSION_OFFSET
}


// pub fn check_client_account(
//     account: &AccountInfo,
//     root_key: Pubkey,
//     program_id: &Pubkey,
//     signer: Pubkey,
//     devol_sign: bool,
// ) -> bool {
//     if devol_sign && client_account_signer_address!(account) != signer {
//         return false;
//     } else if !devol_sign && client_account_owner_address!(account) != signer {
//         return false;
//     }
//     if account.owner != program_id
//         || !account.is_writable
//         || unsafe { *(account.data.borrow().as_ptr() as *const i64) }
//         != version!(CLIENT_ACCOUNT_VERSION, CLIENT_ACCOUNT_TAG)
//     {
//         return false;
//     }
//     return unsafe { *(account.data.borrow()[8..40].as_ptr() as *const Pubkey) } == root_key;
// }
