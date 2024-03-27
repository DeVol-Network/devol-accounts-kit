use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
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

#[derive(PartialEq, PartialOrd, Clone, Copy)]
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


impl DevolAccount for RootAccount {
    #[inline(always)]
    fn expected_size() -> usize { ROOT_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 { ROOT_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { ROOT_ACCOUNT_VERSION }

    #[inline(always)]
    fn check_root(_: AccountTag, _: &AccountHeader, _: &Pubkey) -> Result<(), u32> {
        Ok(())
    }
}

impl RootAccount {
    // pub fn try_from_slice(data: &[u8]) -> Result<Self, Box<dyn Error>> {
    //
    // }
}

impl From<&[u8]> for RootAccount {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), ROOT_ACCOUNT_SIZE, "Incorrect root account size.");
        unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const RootAccount) }
    }
}


// pub fn transform_root_account_safe<'a>(account: &'a AccountInfo, program_id: &Pubkey, writable: bool) -> Result<&'a mut RootAccount, u32> {
//     if account.data.borrow().len() != std::mem::size_of::<RootAccount>() {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountSize));
//     }
//
//     let account_data = transform_account_info::<RootAccount>(account);
//
//     if account.owner != program_id {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountOwner));
//     }
//     if account.is_writable != writable {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountWritableAttribute));
//     }
//     if account_data.header.tag != ROOT_ACCOUNT_TAG as u32 {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountTag));
//     }
//     if account_data.header.version > ROOT_ACCOUNT_VERSION as u32 {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountVersionTooHigh));
//     } else if account_data.header.version < ROOT_ACCOUNT_VERSION as u32 {
//         return Err(error_with_account(AccountTag::Root, ContractError::AccountVersionTooLow));
//     }
//
//     Ok(account_data)
// }


// todo: think over how to cover with transform_account_safe
// impl DevolAccount for RootAccount {
//     #[inline(always)]
//     fn header(&self) -> &AccountHeader { &self.header }
//
//     #[inline(always)]
//     fn expected_version() -> u32 { ROOT_ACCOUNT_VERSION as u32 }
//
//     #[inline(always)]
//     fn expected_tag() -> u8 { ROOT_ACCOUNT_TAG }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_root_account_offsets() {

        let account = RootAccount {
            header: AccountHeader::default(),
            wallet_address: Pubkey::default(),
            kyc_provider: Pubkey::default(),
            mints_address: Pubkey::default(),
            instruments_address: Pubkey::default(),
            workers_address: Pubkey::default(),
            clients_count: 0,
            fee_payer: OpenAccountFeePayer::Client,
            max_light_volume: 0,
        };

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, ROOT_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.wallet_address as *const _ as usize } - base_ptr, ROOT_ACCOUNT_WALLET_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.kyc_provider as *const _ as usize } - base_ptr, ROOT_ACCOUNT_KYC_PROVIDER_OFFSET);
        assert_eq!(unsafe { &account.mints_address as *const _ as usize } - base_ptr, ROOT_ACCOUNT_MINTS_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.instruments_address as *const _ as usize } - base_ptr, ROOT_ACCOUNT_INSTRUMENTS_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.workers_address as *const _ as usize } - base_ptr, ROOT_ACCOUNT_WORKERS_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.clients_count as *const _ as usize } - base_ptr, ROOT_ACCOUNT_CLIENTS_COUNT_OFFSET);
        assert_eq!(unsafe { &account.fee_payer as *const _ as usize } - base_ptr, ROOT_ACCOUNT_KYC_METHOD_OFFSET);
        assert_eq!(unsafe { &account.max_light_volume as *const _ as usize } - base_ptr, ROOT_ACCOUNT_MAX_LIGHT_VOLUME_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<RootAccount>(), ROOT_ACCOUNT_SIZE);
    }
}
