use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::dvl_error::DvlError;
use crate::errors::AccountTag;

pub const ROOT_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const ROOT_ACCOUNT_ADMIN_ADDRESS_OFFSET: usize = 8;
pub const ROOT_ACCOUNT_WALLET_ADDRESS_OFFSET: usize = 40;
pub const ROOT_ACCOUNT_KYC_PROVIDER_OFFSET: usize = 72;
pub const ROOT_ACCOUNT_MINTS_ADDRESS_OFFSET: usize = 104;
pub const ROOT_ACCOUNT_INSTRUMENTS_ADDRESS_OFFSET: usize = 136;
pub const ROOT_ACCOUNT_WORKERS_ADDRESS_OFFSET: usize = 168;
pub const ROOT_ACCOUNT_CLIENTS_COUNT_OFFSET: usize = 200;
pub const ROOT_ACCOUNT_KYC_METHOD_OFFSET: usize = 204;
pub const ROOT_ACCOUNT_MAX_LIGHT_VOLUME_OFFSET: usize = 208;
pub const ROOT_ACCOUNT_SIZE: usize = 216;
pub const ROOT_ACCOUNT_TAG: u8 = 0;
pub const ROOT_ACCOUNT_VERSION: u32 = 2;

#[derive(PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum OpenAccountFeePayer {
    Client = 0,
    Devol = 1,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RootAccount {
    pub header: AccountHeader,             // 40 bytes, ROOT_ACCOUNT_VERSION_OFFSET
    pub wallet_address: Pubkey,            // 32 bytes, ROOT_ACCOUNT_WALLET_ADDRESS_OFFSET
    pub kyc_provider: Pubkey,              // 32 bytes, ROOT_ACCOUNT_KYC_PROVIDER_OFFSET
    pub mints_address: Pubkey,             // 32 bytes, ROOT_ACCOUNT_MINTS_ADDRESS_OFFSET
    pub instruments_address: Pubkey,       // 32 bytes, ROOT_ACCOUNT_INSTRUMENTS_ADDRESS_OFFSET
    pub workers_address: Pubkey,           // 32 bytes, ROOT_ACCOUNT_WORKERS_ADDRESS_OFFSET
    pub clients_count: u32,                //  4 bytes, ROOT_ACCOUNT_CLIENTS_COUNT_OFFSET
    pub fee_payer: OpenAccountFeePayer,    //  4 bytes, ROOT_ACCOUNT_KYC_METHOD_OFFSET
    pub max_light_volume: u64,             //  8 bytes, ROOT_ACCOUNT_MAX_LIGHT_VOLUME_OFFSET
}

impl DevolRegularAccount for RootAccount {}
impl DevolAccount for RootAccount {
    #[inline(always)]
    fn expected_size() -> usize { ROOT_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 { ROOT_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { ROOT_ACCOUNT_VERSION }

    #[inline(always)]
    fn check_root(_: AccountTag, _: &AccountHeader, _: &Pubkey) -> Result<(), DvlError> {
        Ok(())
    }
}

#[cfg(test)]
impl Default for RootAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            wallet_address: Pubkey::default(),
            kyc_provider: Pubkey::default(),
            mints_address: Pubkey::default(),
            instruments_address: Pubkey::default(),
            workers_address: Pubkey::default(),
            clients_count: 0,
            fee_payer: OpenAccountFeePayer::Devol,
            max_light_volume: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_root_account_offsets() {

        let account = RootAccount::default();

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(&account.header as *const _ as usize  - base_ptr, ROOT_ACCOUNT_VERSION_OFFSET);
        assert_eq!(&account.wallet_address as *const _ as usize  - base_ptr, ROOT_ACCOUNT_WALLET_ADDRESS_OFFSET);
        assert_eq!(&account.kyc_provider as *const _ as usize  - base_ptr, ROOT_ACCOUNT_KYC_PROVIDER_OFFSET);
        assert_eq!(&account.mints_address as *const _ as usize  - base_ptr, ROOT_ACCOUNT_MINTS_ADDRESS_OFFSET);
        assert_eq!(&account.instruments_address as *const _ as usize  - base_ptr, ROOT_ACCOUNT_INSTRUMENTS_ADDRESS_OFFSET);
        assert_eq!(&account.workers_address as *const _ as usize  - base_ptr, ROOT_ACCOUNT_WORKERS_ADDRESS_OFFSET);
        assert_eq!(&account.clients_count as *const _ as usize  - base_ptr, ROOT_ACCOUNT_CLIENTS_COUNT_OFFSET);
        assert_eq!(&account.fee_payer as *const _ as usize  - base_ptr, ROOT_ACCOUNT_KYC_METHOD_OFFSET);
        assert_eq!(&account.max_light_volume as *const _ as usize  - base_ptr, ROOT_ACCOUNT_MAX_LIGHT_VOLUME_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<RootAccount>(), ROOT_ACCOUNT_SIZE);
    }
}
