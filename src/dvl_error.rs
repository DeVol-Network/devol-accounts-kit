use std::fmt::Formatter;
use crate::errors::{AccountTag, ContractError};

#[derive(Clone, Copy)]
pub struct DvlError {
    error: ContractError,
    account: Option<AccountTag>,
}



// impl DvlError {
    /// Creates a new `ContractErrorDetails` with a specified error and associated account.
    pub fn new_with_account(account: AccountTag, error: ContractError) -> Self {
        Self {
            error,
            account: Some(account),
        }
    }

    /// Creates a new `ContractErrorDetails` with a specified error without associating an account.
    pub fn new(error: ContractError) -> Self {
        Self {
            error,
            account: None,
        }
    }

    /// Encodes the error details into a 32-bit integer for on-chain error handling.
    ///
    /// The error code is a 32-bit unsigned integer where:
    /// - The most significant bit (MSB) is always set to 1 to indicate a new error system.
    /// - The next 3 bits (from MSB) are reserved for future use and are currently set to 0.
    /// - The following 8 bits represent the account identifier if provided, or are set to 0.
    /// - The least significant 16 bits represent the `ContractError` code.
    ///
    /// # Arguments
    ///
    /// * `error` - A `ContractError` enumeration representing the specific error.
    /// * `account` - An optional `Account` enumeration representing the account involved in the error, if applicable.
    ///
    /// # Returns
    ///
    /// A 32-bit unsigned integer encoding the error and account information according to the rules described.
    ///
    /// # Layout
    ///
    ///  31 30   29 25    24                           16 15                              0
    /// +----+----+-------+------------------------------+--------------------------------+
    /// | 1  | A  | 0 0 0 | Account ID (if provided)     | ContractError Code             |
    /// +----+----+-------+------------------------------+--------------------------------+
    ///
    /// 1 = MSB, always set to indicate a new error system.
    /// A = Account-related flag (set to 1 if the error is account-specific, 0 otherwise).
    /// Reserved = Currently unused bits, set to 0.
    /// Account ID = 8-bit identifier for the account, shifted into bits 23 through 16.
    /// ContractError Code = 16-bit error code, occupying the least significant bits.
    pub fn encode(&self) -> u32 {
        let error_code = self.error as u32;
        let account_code = self.account.map_or(0, |a| a as u32) << 16;
        let sign_bit = 1 << 31;
        let account_related_bit = if self.account.is_some() { 1 << 30 } else { 0 };
        sign_bit | account_related_bit | account_code | error_code
    }

    pub fn from_code(code: u32) -> Self {
        let error_code = (code & 0xFFFF) as u16;
        let account_code = ((code >> 16) & 0xFF) as u8;
        let has_account = (code >> 30) & 1 == 1;

        let error = ContractError::from_u16(error_code);

        let account = if has_account {
            Some(AccountTag::from_u8(account_code))
        } else {
            None
        };
        Self {
            error,
            account,
        }
    }
}

impl std::fmt::Display for DvlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.account {
            Some(account) => write!(f, "Error: {}, Account: {:?}", self.error, account),
            None => write!(f, "Error: {}", self.error),
        }
    }
}

impl std::fmt::Debug for DvlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DvlError {{ error: {}, account: ", self.error)?;
        match self.account {
            Some(account) => write!(f, "{:?}", account),
            None => write!(f, "None"),
        }?;
        write!(f, " }}")
    }
}

impl std::error::Error for DvlError {}

#[cfg(test)]
mod tests {
    use crate::dvl_error::DvlError;
    use super::*;

    #[test]
    fn test_sets_bits_correctly_1() {
        for error in [
            ContractError::NoError,
        ].iter() {
            for account in [
                AccountTag::Root,
                AccountTag::Mints,
                // Add other AccountTag variants as needed
            ].iter() {
                let error_code = DvlError::new_with_account(*account, *error).encode();

                assert_eq!(error_code >> 31, 1, "The most significant bit should always be set.");
                assert_eq!((error_code >> 30) & 1, 1, "The second most significant bit should be set for account-specific errors.");
                assert_eq!(error_code & (0xFF << 16), (*account as u32) << 16, "The account code bits 24-16 should correctly represent the account.");
                assert_eq!(error_code & 0xFFFF, *error as u16 as u32, "The least significant 16 bits should correctly represent the error code.");
            }
        }
    }

    #[test]
    fn test_sets_bits_correctly_2() {
        for error in [
            ContractError::NoError,
        ].iter() {
            let error_code = DvlError::new(*error).encode();

            assert_eq!(error_code >> 31, 1, "The most significant bit should always be set.");
            assert_eq!((error_code >> 30) & 1, 0, "The second most significant bit should not be set for non-account-specific errors.");
            assert_eq!(error_code & (0xFF << 16), 0, "The account code bits 24-16 should be unset for non-account-specific errors.");
            assert_eq!(error_code & 0xFFFF, *error as u16 as u32, "The least significant 16 bits should correctly represent the error code.");
        }
    }

    #[test]
    fn test_decode_error() {
        let error_code = DvlError::new_with_account(AccountTag::AllWorkers, ContractError::AccountOwner).encode();
        let dvl_error = DvlError::from_code(error_code);
        assert_eq!(dvl_error.error, ContractError::AccountOwner);
        assert_eq!(dvl_error.account, Some(AccountTag::AllWorkers));
    }
}
