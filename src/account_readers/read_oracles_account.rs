use std::error::Error;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use crate::dvl_client::dvl_client::DvlClient;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_regular_account::DevolRegularAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::generate_pda::dvl_generate_pda;

#[async_trait]
impl DvlReadable for OraclesAccount {
    type DvlReadParams<'a> = ();

    async fn get_public_key<'a>(
        dvl_client: &DvlClient,
        _params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Pubkey>, Box<dyn Error>> where Self: Sized {
        let oracle_pda = dvl_generate_pda(
            &dvl_client.admin_public_key,
            &dvl_client.oracle_seed,
            &dvl_client.program_id);
        Ok(Box::from(oracle_pda.key))
    }

    async fn read<'a>(
        dvl_client: &DvlClient,
        params: &Self::DvlReadParams<'a>
    ) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &*Self::get_public_key(dvl_client, params).await?;
        let mut rpc_data = dvl_client.rpc_client.get_account(public_key).await?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &dvl_client.root_pda.key,
            &dvl_client.program_id,
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

    #[tokio::test]
    async fn test_read_oracles_account_auto() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let _oracles = client.get_account::<OraclesAccount>(()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_oracles_account_by_public_key() -> Result<(), Box<dyn Error>> {
        let client = setup_devol_client();
        let oracle_pda = dvl_generate_pda(&client.admin_public_key, &client.oracle_seed, &client.program_id);
        let pubkey = &oracle_pda.key;
        let _oracles_account = client.get_account_by_public_key::<OraclesAccount>(pubkey).await?;
        Ok(())
    }
}
