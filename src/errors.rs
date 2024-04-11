use std::fmt::{Debug};
use thiserror::Error;
use crate::accounts::root::root_account::ROOT_ACCOUNT_TAG;
use crate::accounts::worker::worker_account::WORKER_ACCOUNT_TAG;
use crate::accounts::all_workers::all_workers_account::ALL_WORKERS_ACCOUNT_TAG;
use crate::accounts::oracles::oracles_account::ORACLES_ACCOUNT_TAG;
use crate::accounts::instruments::instruments_account::INSTR_ACCOUNT_TAG;
use crate::accounts::worker::pools_log::pools_log_account::POOLS_LOG_ACCOUNT_TAG;
use crate::accounts::worker::tasks_log::tasks_log_account::TASKS_LOG_ACCOUNT_TAG;
use crate::accounts::client::trade_log::trade_log_account::TRADE_LOG_ACCOUNT_TAG;
use crate::accounts::worker::pools_trace::pools_trace_account::POOLS_TRACE_ACCOUNT_TAG;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TASKS_TRACE_ACCOUNT_TAG;
use crate::accounts::mints::mints_account::MINTS_ACCOUNT_TAG;
use crate::accounts::mints::mint_log::mint_log_account::MINT_LOG_ACCOUNT_TAG;
use crate::accounts::client::payoff_log::payoff_log_account::PAYOFF_LOG_ACCOUNT_TAG;
use crate::accounts::client::client_account::client_account::CLIENT_ACCOUNT_TAG;

#[repr(u16)]
#[derive(Copy, Clone, Debug, Error)]
pub enum ContractError {
    #[error("No errors")]
    NoError                     = 0x0000,
    #[error("Incorrect account size")]
    AccountSize                 = 0x0001,
    #[error("The smart contract is not the owner of this account")]
    AccountOwner                = 0x0002,
    #[error("The `isWritable` attribute does not match the expected one passed in the transaction")]
    AccountWritableAttribute    = 0x0003,
    #[error("Account type is different from expected (determined by tag)")]
    WrongAccountTag             = 0x0004,
    #[error("The account version is higher than expected by the smart contract")]
    AccountVersionTooHigh       = 0x0005,
    #[error("The administrator must upgrade this account")]
    AccountVersionTooLow        = 0x0006,
    #[error("The account is not part of the tree of accounts descended from the root account")]
    RootAddress                 = 0x0007,
    #[error("An attempt to resize the account was unsuccessful")]
    IncreaseSize                = 0x0008,
    #[error("Incorrect instruction data length")]
    InstructionDataLength       = 0x0009,
    #[error("For an account with this tag, no actions are provided for this transaction or the tag is invalid")]
    UnknownAccountTag           = 0x000A,
    #[error("Error sending lamports from the signer's account to the target account")]
    LamportTransfer             = 0x000B,
    #[error("Incorrect size, version, writable attribute, etc., only for legacy checks")]
    AccountLegacyError          = 0x000C,
    #[error("Incorrect number of accounts passed to the transaction")]
    AccountsQuantity            = 0x000D,
    #[error("A specific account is missing")]
    AccountIsMissing            = 0x000E,
    #[error("There is a discrepancy between the order of the accounts passed to the Oracle instruction and the saved settings. Accounts must be transferred in the same order as the settings for them are specified.")]
    MismatchOraclesOrder        = 0x000F,
    #[error("The derived PDA does not match the expected account. This may indicate an issue with the seed(s) used for its generation or incorrect account usage.")]
    InvalidPDA                  = 0x0010,
    #[error("The operation could not be completed due to insufficient funds in the involved account(s). This error is raised when an account lacks the necessary lamports to perform actions such as account creation, transaction fees, or lamport transfers.")]
    InsufficientFunds           = 0x0011,
    #[error("Error while creating a new account")]
    CreateAccount               = 0x0012,
    #[error("Such an Oracle does not exist (probably the number is greater than or equal to ORACLES_DATA_COUNT)")]
    SuchOracleDoesNotExists     = 0x0013,
    #[error("Such an Instrument does not exist")]
    SuchInstrumentDoesNotExists = 0x0014,
    #[error("Used outdated instruction data format")]
    InstructionDataVersion      = 0x0015,
    #[error("Operations halted nearing expiration (e.g., maintenance, lockout)")]
    TradeCloseBeforeExpire      = 0x0016,
    #[error("Trade not possible; the option period has ended, next period yet to start (check option schedule)")]
    OptionPeriodClosed          = 0x0017,
    #[error("Option order exceeds 0.5% of liquidity pool (reduce order size)")]
    NotionalValueTooHigh        = 0x0018,
    #[error("Actual trade cost exceeds set maximum limit (adjust limit or price)")]
    CostLimitExceeded           = 0x0019,
    #[error("Transaction exceeds daily trading limit; register as Power Trader to remove this restriction")]
    DailyLimitExceeded          = 0x001A,
    #[error("New pool cannot start without initial funding; deposit funds to launch")]
    PoolFundingRequired         = 0x001B,
    #[error("Cannot determine asset price via oracles; trading halted (oracle not configured or data refresh issue)")]
    AssetPriceUnavailable       = 0x001C,
    #[error("Failed to read current time in smart contract; operation cannot proceed")]
    TimeReadError               = 0x001D,
    #[error("Error during calculations (type conversion, division by zero, floating point overflow, etc.)")]
    ComputationError            = 0x001E,
    #[error("Incorrect oracle number (number too large or negative); oracle does not exist")]
    InvalidOracleNumber         = 0x001F,
    #[error("Transaction must be executed by platform administrator only, but current signer is not authorized")]
    AdminOnlyTransaction        = 0x0020,
    #[error("Transaction requires the account to be a signer, but it is not")]
    AccountNotSigner            = 0x0021,
    #[error("Operation on client account attempted by incorrect client (wrong account sequence, missing, or another client's account)")]
    UnauthorizedClientOperation = 0x0022,
    #[error("No record of the targeted pool in client's account")]
    PoolRecordNotFound          = 0x0023,
    #[error("Significant price variance between oracles exceeds allowable range, preventing trade execution")]
    PriceDiscrepancyError       = 0x0024,
    #[error("Required oracle for price calculation is not provided in the function call")]
    MandatoryOracleMissing      = 0x0025,
    #[error("Invalid account ID")]
    InvalidAccountId            = 0x0026,
    #[error("Invalid mint ID")]
    InvalidMintId               = 0x0027,
    #[error("Operation with worker account cannot proceed due to incorrect state")]
    WorkerInvalidState          = 0x0028,
    #[error("Task cannot start before the current date; adjust task start time")]
    TaskStartBeforeCurrentDate  = 0x0029,
    #[error("Cannot assign new task as the maximum number of workers for the instrument has been reached")]
    MaxWorkersExceeded          = 0x0030,
    #[error("Specified fee payer for account opening does not match any valid options")]
    InvalidFeePayerOption       = 0x0031,
    #[error("Cannot finalize pool as it is still active (worker is active)")]
    PoolActiveCannotFinalize    = 0x0032,
    #[error("Cannot finalize pool as the designated time for finalization has not yet been reached")]
    PoolFinalizeTimeNotReached  = 0x0033,
    #[error("Provided PDA does not match expected for account creation")]
    IncorrectExpectedPDA        = 0x0034,
    #[error("Failed to transfer lamports to newly created account for rent exemption")]
    LamportsTransferFailed      = 0x0035,
    #[error("Created account size does not match the expected size")]
    CreatedAccountSizeMismatch  = 0x0036,
    #[error("Attempting to execute a trade with zero volume is not allowed")]
    ZeroVolumeTradeAttempt      = 0x0037,
    #[error("Trade operation attempted outside allowed activity hours or while pool is inactive")]
    TradeOutsideActivePeriod    = 0x0038,
    #[error("Cannot participate in more than 128 pools simultaneously as LP")]
    MaxPoolsParticipationReached= 0x0039,
    #[error("Attempt to sell more pool tokens than owned")]
    PoolTokenSaleExceedsHoldings= 0x003A,
    #[error("Trade exceeds maximum allowed deposit in pool, cannot proceed")]
    DepositExceedsPoolLimit     = 0x003B,
    #[error("Worker account in transaction does not match the one specified in instruction")]
    WorkerAccountMismatch       = 0x003C,
    #[error("Specified pool index exceeds maximum limit or is not valid")]
    PoolIndexOutOfRange         = 0x003D,
    #[error("Cannot proceed; settlement price not set or pool yet to be finalized")]
    SettlementPriceUnavailable  = 0x003E,
    #[error("Attempt to claim payoff before pool expiration is not allowed")]
    EarlyPayoffAttempt          = 0x003F,
    #[error("Invalid access to pool data for the client or worker account")]
    InvalidPoolAccess           = 0x0040,
    #[error("Worker state is not valid for task assignment; must be in 'Assigned' state")]
    WorkerInvalidStateForTask   = 0x0041,
    #[error("Worker's task duration cannot be zero; set a valid duration")]
    WorkerDurationZero          = 0x0042,
    #[error("Worker's initial offering price cannot be below ID; ensure correct price setup")]
    WorkerInitPxBelowID         = 0x0043,
    #[error("Worker's width factor cannot exceed ID; adjust width factor")]
    WorkerWidthFactorExceedsID  = 0x0044,
    #[error("Worker's fee rate is out of acceptable bounds; ensure fee rate is within limits")]
    WorkerFeeRateOutOfBounds    = 0x0045,
    #[error("Worker's fee ratio is out of acceptable bounds; adjust fee ratio to fit within limits")]
    WorkerFeeRatioOutOfBounds   = 0x0046,
    #[error("Worker's inventories ratio is out of acceptable bounds; ensure inventories ratio is within limits")]
    WorkerInvRatioOutOfBounds   = 0x0047,
    #[error("Transaction must be executed by KYC administrator only, but current signer is not authorized")]
    KycAdminOnlyTransaction     = 0x0048,
    #[error("Token account owner does not match expected SPL Token program")]
    TokenOwnerMismatch          = 0x0049,
    #[error("Token account's mint does not match the specified mint account")]
    TokenMintMismatch           = 0x004A,
    #[error("Account is not initialized")]
    AccountUninitialized        = 0x004B,
    #[error("Token account's public key does not match the expected")]
    TokenPkMismatch             = 0x004C,
    #[error("Program token account does not match the specified program account")]
    ProgramTokenAccMismatch     = 0x004D,
    #[error("Token transfer failed")]
    TransferFailed              = 0x004E,
    #[error("Token transfer execution failed")]
    TransferExecutionFailed     = 0x004F,
    #[error("Insufficient balance for withdrawal")]
    InsufficientBalance         = 0x0050,
    #[error("Attempted to invoke a non-existent smart contract instruction; check the instruction number")]
    InvalidInstruction          = 0x0051,
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