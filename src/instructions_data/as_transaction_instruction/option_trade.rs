use std::error::Error;
use async_trait::async_trait;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use crate::account_readers::dvl_readable::{DvlClientParams, DvlIndexParam};
use crate::accounts::client::client_account::client_account::ClientAccount;
use crate::accounts::instruments::instruments_account::InstrumentsAccount;
use crate::accounts::oracles::oracles_account::OraclesAccount;
use crate::accounts::root::root_account::RootAccount;
use crate::accounts::worker::pools_log::pools_log_account::PoolsLogAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TasksTraceAccount;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::option_trade::InstructionOptionTrade;

pub struct OptionTradeTransactionParams {
    pub worker_id: u32,
    pub client_key: Pubkey,
}

#[async_trait]
impl AsTransactionInstruction for InstructionOptionTrade {
    type DvlTransactionInstructionParams = OptionTradeTransactionParams;

    async fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.to_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(()).await?;
        let client_acc_key = client.account_public_key::<ClientAccount>(DvlClientParams {
            client_address: &transaction_params.client_key,
            signer_account_params: None,
        }).await?;
        let instruments_acc_key = client.account_public_key::<InstrumentsAccount>(()).await?;
        let worker_acc_key = client.account_public_key::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let pools_trace_key = client.account_public_key::<PoolsTraceAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let tasks_trace_key = client.account_public_key::<TasksTraceAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let pools_log_key = client.account_public_key::<PoolsLogAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let worker_account = client.get_account::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let instr_id = worker_account.instr_id;
        let instrument_account = client.get_account::<InstrumentsAccount>(()).await?;
        let oracle_id = instrument_account.data[instr_id as usize].assigned_oracle_num;
        let oracle_acc = client.get_account::<OraclesAccount>(()).await?;
        let oracle_data = oracle_acc.data[oracle_id as usize];
        let oracles_account_key = client.account_public_key::<OraclesAccount>(()).await?;
        let ext_oracle_account1_key = oracle_data.params[0].account;
        let ext_oracle_account2_key = oracle_data.params[1].account;
        let ext_oracle_account3_key = oracle_data.params[2].account;
        let relative_oracle_data = oracle_acc.data[oracle_data.relative_oracle_num as usize];
        let relative_ext_oracle_account1_key =
            relative_oracle_data.params[0].account;
        let relative_ext_oracle_account2_key =
            relative_oracle_data.params[1].account;
        let relative_ext_oracle_account3_key =
            relative_oracle_data.params[2].account;

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
                pubkey: *client_acc_key,
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
                pubkey: *pools_log_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta {
                pubkey: *oracles_account_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: ext_oracle_account1_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: ext_oracle_account2_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: ext_oracle_account3_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: relative_ext_oracle_account1_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: relative_ext_oracle_account2_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: relative_ext_oracle_account3_key,
                is_signer: false,
                is_writable: false,
            },
        ]);
        Ok(Box::from(Instruction::new_with_bytes(
            client.program_id,
            &*data,
            account_metas,
        )))
    }
}
