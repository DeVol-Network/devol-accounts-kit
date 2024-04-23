use std::error::Error;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use crate::account_readers::dvl_readable::DvlIndexParam;
use crate::accounts::root::root_account::RootAccount;
use crate::accounts::worker::pools_trace::pools_trace_account::PoolsTraceAccount;
use crate::accounts::worker::worker_account::WorkerAccount;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::start_next_pool::InstructionStartNextPool;
use crate::transactions::devol_transaction::DevolTransaction;

pub struct StartNextPoolTransactionParams {
    pub worker_id: u32,
}

impl DevolTransaction for InstructionStartNextPool {
    type DvlTransactionParams = StartNextPoolTransactionParams;

    fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Keypair,
        transaction_params: Self::DvlTransactionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.as_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(())?;
        let worker_acc_key = client.account_public_key::<WorkerAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let pools_trace_key = client.account_public_key::<PoolsTraceAccount>(DvlIndexParam { id: transaction_params.worker_id })?;
        let account_metas = Vec::from([
            AccountMeta {
                pubkey: signer.pubkey(),
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