#[repr(u8)]
pub enum Instructions {
    LpTrade = 111,
    OptionTrade = 112,
    Payoff = 113,
    TransferToken = 118,
    WithdrawToken = 119,
    StartPool = 129,
    FinPool = 130,
    StartNextPool = 132,
    OptionTradeDebug = 143,
}