#[repr(u8)]
pub enum Instructions {
    OptionTrade = 112,
    OptionTradeDebug = 143,
    TransferToken = 118,
    WithdrawToken = 119,
    FinPool = 130,
    StartPool = 129,
    LpTrade = 111
}