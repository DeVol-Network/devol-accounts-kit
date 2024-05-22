use solana_program::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct AccountHeader {
    pub tag: u32,
    pub version: u32,
    pub root: Pubkey,
} // 40 bytes

#[cfg(test)]
impl Default for AccountHeader {
    fn default() -> Self {
        Self {
            version: 0,
            tag: 0,
            root: Pubkey::default(),
        }
    }
}