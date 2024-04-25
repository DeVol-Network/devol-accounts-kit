use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use crate::account_readers::dvl_readable::{DvlReadable};
use crate::accounts::devol_account::DevolAccount;
use crate::generate_pda::{generate_pda, PDA};

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
    pub fn new(client: RpcClient, int_seed: usize, admin_public_key_str: &str, program_id_str: &str) -> Self {
        let admin_public_key = Pubkey::from_str(admin_public_key_str).unwrap();
        let program_id = Pubkey::from_str(program_id_str).unwrap();
        let root_seed = format!("rt{}", int_seed);
        let root_pda = generate_pda(&admin_public_key, &root_seed, &program_id);

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

    pub fn get_account<'a, T: DvlReadable>(&self, params: T::DvlReadParams<'a>) -> Result<Box<T>, Box<dyn Error>> {
        T::read(self, &params)
    }

    pub fn get_account_by_public_key<T: DvlReadable + DevolAccount + Copy>(
        &self,
        public_key: &Pubkey,
    ) -> Result<Box<T>, Box<dyn Error>> {
        T::read_by_public_key(self, public_key)
    }

    pub fn account_public_key<'a, T: DvlReadable>(&self, params: T::DvlReadParams<'a>) -> Result<Box<Pubkey>, Box<dyn Error>> {
        T::get_public_key(self, &params)
    }

    pub fn send_transaction(
        &self,
        instruction: Instruction,
        signer_kp: Keypair,
        commitment_config: Option<CommitmentConfig>,
        compute_budget: Option<u32>,
        max_retries: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        let mut instructions = Vec::<Instruction>::new();
        if let Some(max_units) = compute_budget {
            let compute_budget_instruction =
                ComputeBudgetInstruction::set_compute_unit_limit(max_units);
            instructions.push(compute_budget_instruction);
        }
        instructions.push(instruction);

        let commitment = commitment_config.unwrap_or_else(|| self.rpc_client.commitment());

        let retries = max_retries.unwrap_or(1);
        for i in 0..retries {
            let latest_blockhash = self.rpc_client.get_latest_blockhash()?;

            let mut new_transaction = Transaction::new_with_payer(&instructions, Some(&signer_kp.pubkey()));
            new_transaction.try_sign(&[&signer_kp], latest_blockhash)?;

            let send_result = self.rpc_client.send_and_confirm_transaction_with_spinner_and_commitment(
                &new_transaction,
                commitment,
            );

            match send_result {
                Ok(signature) => {
                    println!("Transaction sent successfully with signature: {}", signature);
                    return Ok(());
                },
                Err(_) if i < retries - 1 => continue,
                Err(e) => {
                    eprintln!("Failed to send transaction after {} attempts: {}", retries, e);
                    return Err(Box::new(e));
                }
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "All retry attempts failed",
        )))
    }
}