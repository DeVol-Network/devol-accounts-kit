use std::error::Error;
use std::str::FromStr;
use async_trait::async_trait;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::DvlIndexParam;
use crate::accounts::mints::mint_log::mint_log_account::MintLogAccount;
use crate::accounts::mints::mints_account::MintsAccount;
use crate::accounts::root::root_account::RootAccount;
use crate::constants::TOKEN_PROGRAM_ID;
use crate::dvl_client::dvl_client::DvlClient;
use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::transfer_token::InstructionTransferToken;

pub struct TransferTokenTransactionParams {
    pub mint_id: u32,
    pub client_key: Pubkey,
    pub client_key_pda: Pubkey,
}

#[async_trait]
impl AsTransactionInstruction for InstructionTransferToken {
    type DvlTransactionInstructionParams = TransferTokenTransactionParams;

    async fn as_transaction_instruction(
        &self,
        client: &DvlClient,
        signer: &Pubkey,
        transaction_params: Self::DvlTransactionInstructionParams,
    ) -> Result<Box<Instruction>, Box<dyn Error>> {
        let data = self.to_vec_le();
        let root_acc_key = client.account_public_key::<RootAccount>(()).await?;
        let mint_acc_key = client.account_public_key::<MintsAccount>(()).await?;
        let mints_acc = client.get_account::<MintsAccount>(()).await?;
        let client_acc_key_pda = transaction_params.client_key_pda;
        let client_acc_key = transaction_params.client_key;
        let mint_program_acc_key = mints_acc.data[transaction_params.mint_id as usize].program_address;
        let mint_address_acc_key = mints_acc.data[transaction_params.mint_id as usize].address;
        let log_acc_key = client.account_public_key::<MintLogAccount>(DvlIndexParam { id: transaction_params.mint_id }).await?;
        let token_program_id_key = Pubkey::from_str(TOKEN_PROGRAM_ID)?;
        let associated_token_account = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?;
        let (mint_token_account, _) = Pubkey::find_program_address(&[&client_acc_key.to_bytes(), &token_program_id_key.to_bytes(), &mint_address_acc_key.to_bytes()], &associated_token_account);

        let account_metas = Vec::from([
            AccountMeta {
                pubkey: *signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: mint_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_program_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *root_acc_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: *mint_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: token_program_id_key,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: client_acc_key_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *log_acc_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_address_acc_key,
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