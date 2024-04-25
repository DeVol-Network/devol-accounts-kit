use solana_program::pubkey::Pubkey;

pub struct PDA {
    pub key: Pubkey,
    pub bump_seed: u8,
}

pub fn dvl_generate_pda(
    address: &Pubkey,
    seed_str: &str,
    program_id: &Pubkey,
) -> PDA {
    let mut i = 0;
    let pk: Pubkey;
    loop {
        if let Ok(res) = Pubkey::create_program_address(
            &[seed_str.as_bytes(), address.as_ref(), &[i]],
            &program_id,
        ) {
            pk = res;
            break;
        }
        i += 1;
    }

    PDA {
        key: pk,
        bump_seed: i,
    }
}