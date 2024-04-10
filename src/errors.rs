use std::fmt::{Debug, Formatter};
use crate::accounts::root::root_account::ROOT_ACCOUNT_TAG;
use crate::accounts::worker::worker_account::WORKER_ACCOUNT_TAG;
use crate::accounts::all_workers::all_workers_account::ALL_WORKERS_ACCOUNT_TAG;
use crate::accounts::oracles::oracles_account::ORACLES_ACCOUNT_TAG;
use crate::accounts::instruments::instruments_account::INSTR_ACCOUNT_TAG;
use crate::accounts::worker::pools_log::pools_log_account::POOLS_LOG_ACCOUNT_TAG;
use crate::accounts::worker::task_log::task_log_account::TASKS_LOG_ACCOUNT_TAG;
use crate::accounts::client::trade_log::trade_log_account::TRADE_LOG_ACCOUNT_TAG;
use crate::accounts::worker::pools_trace::pools_trace_account::POOLS_TRACE_ACCOUNT_TAG;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TASKS_TRACE_ACCOUNT_TAG;
use crate::accounts::mints::mints_account::MINTS_ACCOUNT_TAG;
use crate::accounts::mints::mint_log::mint_log_account::MINT_LOG_ACCOUNT_TAG;
use crate::accounts::client::payoff_log::payoff_log_account::PAYOFF_LOG_ACCOUNT_TAG;
use crate::accounts::client::client_account::client_account::CLIENT_ACCOUNT_TAG;

#[repr(u16)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum ContractError {
    NoError                     = 0x0000,   // No errors
    AccountSize                 = 0x0001,   // Account size is smaller than expected; this may be a problem with Solana
    AccountOwner                = 0x0002,   // The smart contract is not the owner of this account
    AccountWritableAttribute    = 0x0003,   // The `isWritable` attribute does not match the expected one passed in the transaction
    WrongAccountTag             = 0x0004,   // Account type is different from expected (determined by tag)
    AccountVersionTooHigh       = 0x0005,   // The account version is higher than expected by the smart contract
    AccountVersionTooLow        = 0x0006,   // The administrator must upgrade this account
    RootAddress                 = 0x0007,   // The account is not part of the tree of accounts descended from the root account
    IncreaseSize                = 0x0008,   // An attempt to resize the account was unsuccessful
    InstructionDataLength       = 0x0009,   // Incorrect instruction data length
    UnknownAccountTag           = 0x000A,   // For an account with this tag, no actions are provided for this transaction or the tag is invalid
    LamportTransfer             = 0x000B,   // Error sending lamports from the signer's account to the target account
    AccountLegacyError          = 0x000C,   // Incorrect size, version, writable attribute, etc., only for legacy checks
    AccountsQuantity            = 0x000D,   // Incorrect number of accounts passed to the transaction
    AccountIsMissing            = 0x000E,   // A specific account is missing
    MismatchOraclesOrder        = 0x000F,   // There is a discrepancy between the order of the accounts passed to the Oracle instruction and the saved settings. Accounts must be transferred in the same order as the settings for them are specified.
    InvalidPDA                  = 0x0010,   // The derived PDA does not match the expected account. This may indicate an issue with the seed(s) used for its generation or incorrect account usage.
    InsufficientFunds           = 0x0011,   // The operation could not be completed due to insufficient funds in the involved account(s). This error is raised when an account lacks the necessary lamports to perform actions such as account creation, transaction fees, or lamport transfers.
    CreateAccount               = 0x0012,   // Error while creating a new account
    SuchOracleDoesNotExists     = 0x0013,   // Such an Oracle does not exist (probably the number is greater than or equal to ORACLES_DATA_COUNT)
    SuchInstrumentDoesNotExists = 0x0014,   // Such an Instrument does not exist
    InstructionDataVersion      = 0x0015,   // Used outdated instruction data format
    TradeCloseBeforeExpire      = 0x0016,   // Operations halted nearing expiration (e.g., maintenance, lockout)
    OptionPeriodClosed          = 0x0017,   // Trade not possible; the option period has ended, next period yet to start (check option schedule)
    NotionalValueTooHigh        = 0x0018,   // Option order exceeds 0.5% of liquidity pool (reduce order size)
    CostLimitExceeded           = 0x0019,   // Actual trade cost exceeds set maximum limit (adjust limit or price)
    DailyLimitExceeded          = 0x001A,   // Transaction exceeds daily trading limit; register as Power Trader to remove this restriction
    PoolFundingRequired         = 0x001B,   // New pool cannot start without initial funding; deposit funds to launch
    AssetPriceUnavailable       = 0x001C,   // Cannot determine asset price via oracles; trading halted (oracle not configured or data refresh issue)
    TimeReadError               = 0x001D,   // Failed to read current time in smart contract; operation cannot proceed
    ComputationError            = 0x001E,   // Error during calculations (type conversion, division by zero, floating point overflow, etc.)
    InvalidOracleNumber         = 0x001F,   // Incorrect oracle number (number too large or negative); oracle does not exist
    AdminOnlyTransaction        = 0x0020,   // Transaction must be executed by platform administrator only, but current signer is not authorized
    AccountNotSigner            = 0x0021,   // Transaction requires the account to be a signer, but it is not
    UnauthorizedClientOperation = 0x0022,   // Operation on client account attempted by incorrect client (wrong account sequence, missing, or another client's account)
    PoolRecordNotFound          = 0x0023,   // No record of the targeted pool in client's account
    PriceDiscrepancyError       = 0x0024,   // Significant price variance between oracles exceeds allowable range, preventing trade execution
    MandatoryOracleMissing      = 0x0025,   // Required oracle for price calculation is not provided in the function call
    InvalidAccountId            = 0x0026,   // Invalid account ID
    InvalidMintId               = 0x0027,   // Invalid mint ID
    WorkerInvalidState          = 0x0028,   // Operation with worker account cannot proceed due to incorrect state
    TaskStartBeforeCurrentDate  = 0x0029,   // Task cannot start before the current date; adjust task start time
    MaxWorkersExceeded          = 0x0030,   // Cannot assign new task as the maximum number of workers for the instrument has been reached
    InvalidFeePayerOption       = 0x0031,   // Specified fee payer for account opening does not match any valid options
    PoolActiveCannotFinalize    = 0x0032,   // Cannot finalize pool as it is still active (worker is active)
    PoolFinalizeTimeNotReached  = 0x0033,   // Cannot finalize pool as the designated time for finalization has not yet been reached
    IncorrectExpectedPDA        = 0x0034,   // Provided PDA does not match expected for account creation
    LamportsTransferFailed      = 0x0035,   // Failed to transfer lamports to newly created account for rent exemption
    CreatedAccountSizeMismatch  = 0x0036,   // Created account size does not match the expected size
    ZeroVolumeTradeAttempt      = 0x0037,   // Attempting to execute a trade with zero volume is not allowed
    TradeOutsideActivePeriod    = 0x0038,   // Trade operation attempted outside allowed activity hours or while pool is inactive
    MaxPoolsParticipationReached= 0x0039,   // Cannot participate in more than 128 pools simultaneously as LP
    PoolTokenSaleExceedsHoldings= 0x003A,   // Attempt to sell more pool tokens than owned
    DepositExceedsPoolLimit     = 0x003B,   // Trade exceeds maximum allowed deposit in pool, cannot proceed
    WorkerAccountMismatch       = 0x003C,   // Worker account in transaction does not match the one specified in instruction
    PoolIndexOutOfRange         = 0x003D,   // Specified pool index exceeds maximum limit or is not valid
    SettlementPriceUnavailable  = 0x003E,   // Cannot proceed; settlement price not set or pool yet to be finalized
    EarlyPayoffAttempt          = 0x003F,   // Attempt to claim payoff before pool expiration is not allowed
    InvalidPoolAccess           = 0x0040,   // Invalid access to pool data for the client or worker account
    WorkerInvalidStateForTask   = 0x0041,   // Worker state is not valid for task assignment; must be in 'Assigned' state
    WorkerDurationZero          = 0x0042,   // Worker's task duration cannot be zero; set a valid duration
    WorkerInitPxBelowID         = 0x0043,   // Worker's initial offering price cannot be below ID; ensure correct price setup
    WorkerWidthFactorExceedsID  = 0x0044,   // Worker's width factor cannot exceed ID; adjust width factor
    WorkerFeeRateOutOfBounds    = 0x0045,   // Worker's fee rate is out of acceptable bounds; ensure fee rate is within limits
    WorkerFeeRatioOutOfBounds   = 0x0046,   // Worker's fee ratio is out of acceptable bounds; adjust fee ratio to fit within limits
    WorkerInvRatioOutOfBounds   = 0x0047,   // Worker's inventories ratio is out of acceptable bounds; ensure inventories ratio is within limits
    KycAdminOnlyTransaction     = 0x0048,   // Transaction must be executed by KYC administrator only, but current signer is not authorized
    TokenOwnerMismatch          = 0x0049,   // Token account owner does not match expected SPL Token program
    TokenMintMismatch           = 0x004A,   // Token account's mint does not match the specified mint account
    AccountUninitialized        = 0x004B,   // Account is not initialized
    TokenPkMismatch             = 0x004C,   // Token account's public key does not match the expected
    ProgramTokenAccMismatch     = 0x004D,   // Program token account does not match the specified program account
    TransferFailed              = 0x004E,   // Token transfer failed

}

#[allow(dead_code)]
pub fn decode_error_code(error_code: u32) -> String {
    let is_new_system = (error_code >> 31) & 1 == 1;
    if !is_new_system {
        return format!("Legacy error code: {}", error_code);
    }

    let account_code = (error_code >> 16) & 0xFF;
    let contract_error_code = error_code & 0xFFFF;

    let error_description = match contract_error_code {
        0x0000 => "No errors",
        0x0001 => "Account size is smaller than expected; this may be a problem with Solana",
        0x0002 => "The smart contract is not the owner of this account",
        0x0003 => "The `isWritable` attribute does not match the expected one passed in the transaction",
        0x0004 => "Account type is different from expected (determined by tag)",
        0x0005 => "The account version is higher than expected by the smart contract",
        0x0006 => "The administrator must upgrade this account",
        0x0007 => "The account is not part of the tree of accounts descended from the root account",
        0x0008 => "An attempt to resize the account was unsuccessful",
        0x0009 => "Incorrect instruction data length",
        0x000A => "For an account with this tag, no actions are provided for this transaction or the tag is invalid",
        0x000B => "Error sending lamports from the signer's account to the target account",
        0x000C => "Incorrect size, version, writable attribute, etc., only for legacy checks",
        0x000D => "Incorrect number of accounts passed to the transaction",
        0x000E => "A specific account is missing",
        0x000F => "There is a discrepancy between the order of the accounts passed to the Oracle instruction and the saved settings",
        0x0010 => "The derived PDA does not match the expected account",
        0x0011 => "The operation could not be completed due to insufficient funds in the involved account(s)",
        0x0012 => "Error while creating a new account",
        0x0013 => "Such an Oracle does not exist",
        0x0014 => "Such an Instrument does not exist",
        0x0015 => "Used outdated instruction data format",
        0x0016 => "Operations halted nearing expiration",
        0x0017 => "Trade not possible; the option period has ended",
        0x0018 => "Option order exceeds 0.5% of liquidity pool",
        0x0019 => "Actual trade cost exceeds set maximum limit",
        0x001A => "Transaction exceeds daily trading limit",
        0x001B => "New pool cannot start without initial funding",
        0x001C => "Cannot determine asset price via oracles",
        0x001D => "Failed to read current time in smart contract",
        0x001E => "Error during calculations",
        0x001F => "Incorrect oracle number",
        0x0020 => "Transaction must be executed by platform administrator only",
        0x0021 => "Transaction requires the account to be a signer, but it is not",
        0x0022 => "Operation on client account attempted by incorrect client (wrong account sequence, missing, or another client's account)",
        0x0023 => "No record of the targeted pool in client's account",
        0x0024 => "Significant price variance between oracles exceeds allowable range, preventing trade execution",
        0x0025 => "Required oracle for price calculation is not provided in the function call",
        0x0026 => "Invalid account ID",
        0x0027 => "Invalid mint ID",
        0x0028 => "Operation with worker account cannot proceed due to incorrect state",
        0x0029 => "Task cannot start before the current date; adjust task start time",
        0x0030 => "Cannot assign new task as the maximum number of workers for the instrument has been reached",
        0x0031 => "Specified fee payer for account opening does not match any valid options",
        0x0032 => "Cannot finalize pool as it is still active (worker is active)",
        0x0033 => "Cannot finalize pool as the designated time for finalization has not yet been reached",
        0x0034 => "Provided PDA does not match expected for account creation",
        0x0035 => "Failed to transfer lamports to newly created account for rent exemption",
        0x0036 => "Created account size does not match the expected size",
        0x0037 => "Attempting to execute a trade with zero volume is not allowed",
        0x0038 => "Trade operation attempted outside allowed activity hours or while pool is inactive",
        0x0039 => "Cannot participate in more than 128 pools simultaneously as LP",
        0x003A => "Attempt to sell more pool tokens than owned",
        0x003B => "Trade exceeds maximum allowed deposit in pool, cannot proceed",
        0x003C => "Worker account in transaction does not match the one specified in instruction",
        0x003D => "Specified pool index exceeds maximum limit or is not valid",
        0x003E => "Cannot proceed; settlement price not set or pool yet to be finalized",
        0x003F => "Attempt to claim payoff before pool expiration is not allowed",
        0x0040 => "Invalid access to pool data for the client or worker account",
        0x0041 => "Worker state is not valid for task assignment; must be in 'Assigned' state",
        0x0042 => "Worker's task duration cannot be zero; set a valid duration",
        0x0043 => "Worker's initial offering price cannot be below ID; ensure correct price setup",
        0x0044 => "Worker's width factor cannot exceed ID; adjust width factor",
        0x0045 => "Worker's fee rate is out of acceptable bounds; ensure fee rate is within limits",
        0x0046 => "Worker's fee ratio is out of acceptable bounds; adjust fee ratio to fit within limits",
        0x0047 => "Worker's inventories ratio is out of acceptable bounds; ensure inventories ratio is within limits",
        0x0048 => "Transaction must be executed by KYC administrator only, but current signer is not authorized",
        0x0049 => "Token account owner does not match expected SPL Token program",
        0x004A => "Token account's mint does not match the specified mint account",
        0x004B => "Account is not initialized",
        0x004C => "Token account's public key does not match the expected",
        0x004D => "Program token account does not match the specified program account",
        0x004E => "Token transfer failed",

        _ => "Unknown error",
    };

    let account_related = (error_code >> 30) & 1 == 1;
    if account_related {
        let account_name = match AccountTag::from_u8(account_code as u8) {
            Some(account_tag) => format!("{:?}", account_tag),
            None => "Unknown account".to_string(),
        };
        format!("Error with account {}: {}", account_name, error_description)
    } else {
        format!("Error: {}", error_description)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum AccountTag {
    // Equals to the account tags (WORKER_ACCOUNT_TAG, ROOT_ACCOUNT_TAG etc.)
    Root                = 0x00,
    Mints               = 0x01,
    Instruments         = 0x02,
    AllWorkers          = 0x03,
    PoolsTrace          = 0x04,
    TasksTrace          = 0x05,
    PoolsLog            = 0x06,
    Worker              = 0x07,
    Client              = 0x08,
    PayoffLog           = 0x09,
    MintLog             = 0x0A,
    TasksLog            = 0x0B,
    TradeLog            = 0x0C,
    Oracle              = 0x0D,
    // Other accounts
    Admin               = 0x0E,   // Connected admin wallet
    SystemProgram       = 0x0F,   // PublicKey(0)
    Target              = 0x10,   // For the case when the target account will be determined at runtime
    ExternalOracle      = 0x11,
    Wallet              = 0x12,   // Connected wallet account
    KycAdmin            = 0x13,   // Connected KYC admin wallet
    ClientToken         = 0x14,
    ProgramToken        = 0x15,
}

impl AccountTag {
    pub fn from_u8(value: u8) -> Option<AccountTag> {
        match value {
            ROOT_ACCOUNT_TAG => Some(AccountTag::Root),
            INSTR_ACCOUNT_TAG => Some(AccountTag::Instruments),
            ALL_WORKERS_ACCOUNT_TAG => Some(AccountTag::AllWorkers),
            WORKER_ACCOUNT_TAG => Some(AccountTag::Worker),
            ORACLES_ACCOUNT_TAG => Some(AccountTag::Oracle),
            MINTS_ACCOUNT_TAG => Some(AccountTag::Mints),
            POOLS_TRACE_ACCOUNT_TAG => Some(AccountTag::PoolsTrace),
            TASKS_TRACE_ACCOUNT_TAG => Some(AccountTag::TasksTrace),
            POOLS_LOG_ACCOUNT_TAG => Some(AccountTag::PoolsLog),
            PAYOFF_LOG_ACCOUNT_TAG => Some(AccountTag::PayoffLog),
            MINT_LOG_ACCOUNT_TAG => Some(AccountTag::MintLog),
            TASKS_LOG_ACCOUNT_TAG => Some(AccountTag::TasksLog),
            TRADE_LOG_ACCOUNT_TAG => Some(AccountTag::TradeLog),
            CLIENT_ACCOUNT_TAG => Some(AccountTag::Client),
            _ => None,
        }
    }
}

/// Constructs an error code from a `ContractError` and an optional `Account`.
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

#[inline(always)]
fn make_error(error: ContractError, account: Option<AccountTag>) -> u32 {
    let error_code = error as u32;
    let account_code = account.map_or(0, |a| a as u8 as u32) << 16; // Сдвигаем на 16, а не на 24
    let sign_bit = 1 << 31;
    let full_error = sign_bit | account_code | error_code;
    full_error
}


/// Generates a 32-bit error code for account-specific errors.
///
/// Combines a `ContractError` with an `Account` to form a unique error code, where the error is associated with a specific account.
///
/// # Arguments
///
/// * `account` - The account related to the error.
/// * `error` - The specific error from `ContractError`.
///
/// # Returns
///
/// A 32-bit error code.

#[inline(always)]
pub fn error_with_account(account: AccountTag, error: ContractError) -> u32 {
    let account_related_bit = 1 << 30;
    make_error(error, Some(account)) | account_related_bit
}

/// Generates a 32-bit error code for general errors not associated with a specific account.
///
/// Encodes a `ContractError` into a 32-bit error code, omitting account-specific information.
///
/// # Arguments
///
/// * `error` - The specific error from `ContractError`.
///
/// # Returns
///
/// A 32-bit error code.

#[inline(always)]
pub fn error_common(error: ContractError) -> u32 {
    make_error(error, None)
}

#[derive(Debug, Clone, Copy)]
pub struct ReadAccountError(u32);

impl std::fmt::Display for ReadAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", decode_error_code(self.0))
    }
}

impl std::error::Error for ReadAccountError {}

impl From<u32> for ReadAccountError {
    fn from(code: u32) -> Self {
        ReadAccountError(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_with_account_sets_bits_correctly() {
        for error in [
            ContractError::NoError,
            ContractError::AccountSize,
            // Add other ContractError variants as needed
        ].iter() {
            for account in [
                AccountTag::Root,
                AccountTag::Mints,
                // Add other AccountTag variants as needed
            ].iter() {
                let error_code = error_with_account(*account, *error);

                assert_eq!(error_code >> 31, 1, "The most significant bit should always be set.");
                assert_eq!((error_code >> 30) & 1, 1, "The second most significant bit should be set for account-specific errors.");
                assert_eq!(error_code & (0xFF << 16), (*account as u32) << 16, "The account code bits 24-16 should correctly represent the account.");
                assert_eq!(error_code & 0xFFFF, *error as u16 as u32, "The least significant 16 bits should correctly represent the error code.");
            }
        }
    }

    #[test]
    fn test_error_common_sets_bits_correctly() {
        for error in [
            ContractError::NoError,
            ContractError::AccountSize,
            // Add other ContractError variants as needed
        ].iter() {
            let error_code = error_common(*error);

            assert_eq!(error_code >> 31, 1, "The most significant bit should always be set.");
            assert_eq!((error_code >> 30) & 1, 0, "The second most significant bit should not be set for non-account-specific errors.");
            assert_eq!(error_code & (0xFF << 16), 0, "The account code bits 24-16 should be unset for non-account-specific errors.");
            assert_eq!(error_code & 0xFFFF, *error as u16 as u32, "The least significant 16 bits should correctly represent the error code.");
        }
    }

    #[test]
    fn legacy_error_code() {
        let legacy_error_code = 109u32;
        assert_eq!(
            decode_error_code(legacy_error_code),
            "Legacy error code: 109",
            "Should return a legacy error message with the code"
        );
    }

    #[test]
    fn common_error_insufficient_funds() {
        let error_code = error_common(ContractError::InsufficientFunds);
        assert_eq!(
            decode_error_code(error_code),
            "Error: The operation could not be completed due to insufficient funds in the involved account(s)",
            "Should return a message for InsufficientFunds error"
        );
    }

    #[test]
    fn account_specific_error_account_tag() {
        let error_code = error_with_account(AccountTag::Worker, ContractError::WrongAccountTag);
        assert_eq!(
            decode_error_code(error_code),
            "Error with account Worker: Account type is different from expected (determined by tag)",
            "Should return an account-specific message for AccountTag error"
        );
    }
}
