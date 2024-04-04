#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ClientSignMethod {
    Wallet = 0,
    SignerAccount = 1,
}