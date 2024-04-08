use crate::accounts::client::client_account::client_account::{CLIENT_ACCOUNT_TAG, CLIENT_ACCOUNT_VERSION};
use crate::accounts::devol_account::DevolAccount;

pub trait ClientDevolAccount: DevolAccount{
    #[inline(always)]
    fn expected_size() -> usize { 0 }

    #[inline(always)]
    fn expected_tag() -> u8 { CLIENT_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { CLIENT_ACCOUNT_VERSION as u32 }
}