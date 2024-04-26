use std::error::Error;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::generate_pda::dvl_generate_pda;


impl DvlReadable for OraclesAccount {
    type DvlReadParams<'a> = ();

    fn get_public_key<'a>(client: &DvlClient, _params: &Self::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let oracle_pda = dvl_generate_pda(&client.admin_public_key, &client.oracle_seed, &client.program_id);
        Ok(Box::from(oracle_pda.key))
    }

    fn read<'a>(client: &DvlClient, params: &Self::DvlReadParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(client, params)?;
        let mut rpc_data = client.rpc_client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &client.root_pda.key,
            &client.program_id,
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::oracles::oracles_account::OraclesAccount;
    use crate::generate_pda::dvl_generate_pda;
    use crate::tests::tests::setup_devol_client;
    use std::error::Error;

    #[test]
    fn test_read_oracles_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _oracles = client.get_account::<OraclesAccount>(());
        Ok(())
    }

    #[test]
    fn test_read_oracles_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let oracle_pda = dvl_generate_pda(&client.admin_public_key, &client.oracle_seed, &client.program_id);
        let pubkey = &oracle_pda.key;
        let _oracles_account = client.get_account_by_public_key::<OraclesAccount>(pubkey)?;
        Ok(())
    }
}
