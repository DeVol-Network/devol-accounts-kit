use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;
use crate::account_readers::dvl_readable::{ClientRelativeAccountParams, DvlReadable};
use crate::accounts::client::payoff_log::payoff_log_account::PayoffLogAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;

impl DvlReadable for PayoffLogAccount {
    type AdditionalCheckParams<'a> = ClientRelativeAccountParams <'a>;

    fn read<'a>(reader: &DvlAccountReader, params: Self::AdditionalCheckParams<'a>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let public_key = &params.client_account.payoff_log;
        let mut rpc_data = reader.client.get_account(public_key)?;
        let account = Self::from_account(
            public_key,
            &mut rpc_data,
            &reader.root_pda.key,
            &reader.program_id,
            Some(params.client_account.id),
        )?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::account_readers::dvl_readable::SignableAccountParams;
    use crate::accounts::client::client_account::client_account::ClientAccount;
    use crate::accounts::client::payoff_log::payoff_log_account::{PAYOFF_LOG_ACCOUNT_TAG, PAYOFF_LOG_ACCOUNT_VERSION};
    use crate::generate_pda::generate_pda;
    use super::*;
    use crate::tests::tests::setup_account_reader;

    #[test]
    fn test_read_payoff_log_account() {
        let reader = setup_account_reader();
        let client_pda = generate_pda(&reader.admin_public_key, &reader.main_seed, &reader.program_id);
        // Test auto read
        let client_account = reader.read::<ClientAccount>(SignableAccountParams {
            client_address: &client_pda.key,
            signer_account_params: None,
        }).unwrap();

        let payoff =
            reader.read::<PayoffLogAccount>(ClientRelativeAccountParams {client_account: &client_account }).unwrap();
        check_payoff_log_account(&payoff);

        // Test read by public key
        let payoff =
            reader.read_by_public_key::<PayoffLogAccount>(&client_account.payoff_log).unwrap();
        check_payoff_log_account(&payoff);
    }

    fn check_payoff_log_account(payoff_log_account: &PayoffLogAccount) {
        assert_eq!(payoff_log_account.header.tag, PAYOFF_LOG_ACCOUNT_TAG as u32);
        assert_eq!(payoff_log_account.header.version, PAYOFF_LOG_ACCOUNT_VERSION);
    }
}
