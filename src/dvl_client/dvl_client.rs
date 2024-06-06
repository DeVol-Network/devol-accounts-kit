use std::error::Error;
use solana_client::client_error::ClientErrorKind;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_program::hash::Hash;
use solana_program::instruction::{Instruction, InstructionError};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::transaction::{Transaction, TransactionError};
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_account::DevolAccount;
use crate::dvl_error::DvlError;
use crate::generate_pda::{dvl_generate_pda, PDA};

pub struct DvlClient {
    pub rpc_client: RpcClient,
    pub int_seed: usize,
    pub admin_public_key: Pubkey,
    pub program_id: Pubkey,
    pub main_seed: String,
    pub log_seed: String,
    pub root_seed: String,
    pub oracle_seed: String,
    pub root_pda: PDA,
}

impl DvlClient {
    pub fn new(client: RpcClient, int_seed: usize, admin_public_key: Pubkey, program_id: Pubkey) -> Self {
        let root_seed = format!("rt{}", int_seed);
        let root_pda = dvl_generate_pda(&admin_public_key, &root_seed, &program_id);

        Self {
            rpc_client: client,
            int_seed,
            admin_public_key,
            program_id,
            main_seed: format!("cm{}", int_seed),
            log_seed: format!("pol{}", int_seed),
            root_seed,
            oracle_seed: format!("orcl{}", int_seed),
            root_pda,
        }
    }

    pub async fn get_account<'a, T: DvlReadable>(
        &self,
        params: T::DvlReadParams<'a>,
    ) -> Result<Box<T>, Box<dyn Error>> {
        T::read(self, &params).await
    }

    pub async fn get_account_by_public_key<T: DvlReadable + DevolAccount + Copy + Send>(
        &self,
        public_key: &Pubkey,
    ) -> Result<Box<T>, Box<dyn Error>> {
        T::read_by_public_key(self, public_key).await
    }

    pub async fn account_public_key<'a, T: DvlReadable>(
        &self,
        params: T::DvlReadParams<'a>,
    ) -> Result<Box<Pubkey>, Box<dyn Error>> {
        T::get_public_key(self, &params).await
    }

    pub async fn send_transaction(
        &self,
        mut params: DvlSendTransactionParams<'_>,
    ) -> Result<String, Box<dyn Error>> {
        if let Some(max_units) = params.compute_budget {
            let compute_budget_instruction = ComputeBudgetInstruction::set_compute_unit_limit(max_units);
            params.instructions.push(compute_budget_instruction);
        }
        if let Some(compute_unit_price) = params.compute_unit_price {
            let priority_fee_instruction = ComputeBudgetInstruction::set_compute_unit_price(compute_unit_price);
            params.instructions.push(priority_fee_instruction);
        }

        let commitment = params.commitment_config.unwrap_or_else(|| self.rpc_client.commitment());
        let retries = params.max_retries.unwrap_or(1);
        let delay = params.retry_delay.unwrap_or(1);
        let log_prefix = params.log_prefix.unwrap_or("");
        let verbose = params.verbose.unwrap_or(false);

        for i in 0..retries {
            let latest_blockhash = self.rpc_client.get_latest_blockhash_with_commitment(
                CommitmentConfig { commitment: CommitmentLevel::Finalized }
            ).await.map_err(|e| Box::new(e) as Box<dyn Error>)?.0;
            let mut new_transaction = Transaction::new_with_payer(&params.instructions, Some(&params.signer));
            (params.signer_fn)(&mut new_transaction, latest_blockhash)?;

            let send_result = self.rpc_client.send_and_confirm_transaction_with_spinner_and_commitment(&new_transaction, commitment).await;

            match send_result {
                Ok(signature) => {
                    if verbose {
                        println!("[INFO] {}Transaction sent successfully with signature: {}", log_prefix, signature);
                    }
                    return Ok(signature.to_string());
                }
                Err(e) if i < retries - 1 => {
                    if verbose {
                        println!("[INFO] {}Retrying transaction due to error: {:?}", log_prefix, e);
                    }
                    match e.kind {
                        ClientErrorKind::RpcError(RpcError::RpcResponseError { code, message, data }) => {
                            if let RpcResponseErrorData::SendTransactionPreflightFailure(sim_result) = data {
                                if let Some(TransactionError::InstructionError(index, InstructionError::Custom(error_code))) = sim_result.err {
                                    let dvl_error = DvlError::from_code(error_code);
                                    println!("[ERROR] {}Custom error in instruction at index {}: {}", log_prefix, index, dvl_error);
                                    return Err(Box::new(dvl_error));
                                } else {
                                    println!("[ERROR] {}Transaction simulation failed with error: {:?}", log_prefix, sim_result.err);
                                }
                            } else {
                                println!("[ERROR] {}RPC error: {}, code: {}, message: {}", log_prefix, code, message, data);
                            }
                        }
                        _ => println!("[ERROR] {}Unexpected error kind: {:?}", log_prefix, e.kind),
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
                }
                Err(e) => {
                    if verbose {
                        eprintln!("[ERROR] {}Failed to send transaction after {} attempts: {}", log_prefix, retries, e);
                    }
                    return Err(Box::new(e));
                }
            }
        }

        if verbose {
            eprintln!("[ERROR] {}All retry attempts failed", log_prefix);
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "All retry attempts failed")))
    }
}

pub type SignerFunction = Box<dyn Fn(&mut Transaction, Hash) -> Result<(), Box<dyn Error>> + Send + Sync>;

pub struct DvlSendTransactionParams<'a> {
    pub instructions: Vec<Instruction>,
    pub signer: &'a Pubkey,
    pub signer_fn: SignerFunction,
    pub commitment_config: Option<CommitmentConfig>,
    pub compute_budget: Option<u32>,
    pub compute_unit_price: Option<u64>,
    pub verbose: Option<bool>,
    pub log_prefix: Option<&'a str>,
    pub max_retries: Option<usize>,
    pub retry_delay: Option<u64>,
}