use std::error::Error;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Signer};
use crate::account_readers::dvl_readable::DvlIndexParam;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::root::root_account::RootAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;
use crate::accounts::worker::tasks_log::tasks_log_account::TasksLogAccount;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TasksTraceAccount;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::lp_trade::InstructionLpTrade;

pub struct LpTradeTransactionParams {
    pub worker_id: u32,
    pub client_key: Pubkey,
}


impl AsTransactionInstruction for InstructionLpTrade {
    type DvlTransactionInstructionParams = LpTradeTransactionParams;

    fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.to_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(())?;
        let instruments_acc_key = client.account_public_key::<InstrumentsAccount>(())?;
        let client_acc_key = transaction_params.client_key;
        let worker_acc_key = client.account_public_key::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let pools_trace_key = client.account_public_key::<PoolsTraceAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let tasks_trace_key = client.account_public_key::<TasksTraceAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let tasks_log_key = client.account_public_key::<TasksLogAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
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
                pubkey: *instruments_acc_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: client_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *worker_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *tasks_trace_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *pools_trace_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *tasks_log_key,
                is_signer: false,
                is_writable: true,
            }
        ]);
        Ok(Box::from(Instruction::new_with_bytes(
            client.program_id,
            &*data,
            account_metas,
        )))
    }
}