use solana_program::pubkey::Pubkey;

pub struct SignerAccountParams<'a> {
    pub signer: &'a Pubkey,
    pub devol_sign: bool,
}