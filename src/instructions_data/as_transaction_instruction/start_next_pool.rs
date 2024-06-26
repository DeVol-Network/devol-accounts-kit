use std::error::Error;
use async_trait::async_trait;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::DvlIndexParam;
use crate::accounts::root::root_account::RootAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::start_next_pool::InstructionStartNextPool;

pub struct StartNextPoolTransactionParams {
    pub worker_id: u32,
}

#[async_trait]
impl AsTransactionInstruction for InstructionStartNextPool {
    type DvlTransactionInstructionParams = StartNextPoolTransactionParams;

    async fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.to_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(()).await?;
        let worker_acc_key = client.account_public_key::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
        let pools_trace_key = client.account_public_key::<PoolsTraceAccount>(DvlIndexParam { id: transaction_params.worker_id }).await?;
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
                pubkey: *worker_acc_key,
                is_signer: false,
                is_writable: true,
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