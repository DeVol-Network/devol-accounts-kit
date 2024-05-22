#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum ClientSignMethod {
    Wallet = 0,
    SignerAccount = 1,
}