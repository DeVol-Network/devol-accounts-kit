use std::error::Error;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::{DvlClientParams, DvlIndexParam};
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::client::client_account::signer_account_params::SignerAccountParams;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::root::root_account::RootAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::payoff::InstructionPayoff;

pub struct PayoffTransactionParams {
    pub worker_id: u32,
    pub client_key: Pubkey,
}


impl AsTransactionInstruction for InstructionPayoff {
    type DvlTransactionInstructionParams = PayoffTransactionParams;

    fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.to_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(())?;
        let client_acc_key = transaction_params.client_key;
        let devol_sign_flag = true;
        let signer_account_params = SignerAccountParams {
            signer: &client_acc_key,
            devol_sign: devol_sign_flag,
        };
        let signer_account_params_option: Option<&SignerAccountParams> = Some(&signer_account_params);
        let client_acc = client.get_account::<ClientAccount>(DvlClientParams { client_address: &transaction_params.client_key, signer_account_params: signer_account_params_option })?;
        let instruments_acc_key = client.account_public_key::<InstrumentsAccount>(())?;
        let worker_acc_key = client.account_public_key::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let pools_trace_key = client.account_public_key::<PoolsTraceAccount>(DvlIndexParam { id: transaction_params.worker_id })?;

        let account_metas = Vec::from([
            AccountMeta {
                pubkey: *signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: *root_acc_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: client_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: client_acc.payoff_log,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *instruments_acc_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: *worker_acc_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: *pools_trace_key,
                is_signer: false,
                is_writable: true,
            },
        ]);
        Ok(Box::from(Instruction::new_with_bytes(
            client.program_id,
            &*data,
            account_metas,
        )))
    }
}