// use std::error::Error;
// use solana_program::pubkey::Pubkey;
// use crate::account_readers::dvl_account_reader::DvlAccountReader;
// use crate::account_readers::dvl_readable::{DvlReadable, IndexedAccountParams};
// use crate::accounts::all_workers::all_workers_account::AllWorkersAccount;
// use crate::accounts::devol_account::DevolAccount;
// use crate::accounts::devol_indexed_account::DevolIndexedAccount;
// use crate::accounts::devol_regular_account::DevolRegularAccount;
// use crate::accounts::worker::worker_account::WorkerAccount;
//
// pub trait DvlReadableIndexed: DvlReadable<AdditionalCheckParams=IndexedAccountParams> {
//     fn read_by_public_key(
//         reader: &DvlAccountReader,
//         public_key: &Pubkey,
//         params: Option<Self::AdditionalCheckParams>,
//     ) -> Result<Box<Self>, Box<dyn Error>>
//         where
//             Self: Sized + DevolAccount + DvlReadableIndexed + Copy
//     {
//         let mut rpc_data = reader.client.get_account(public_key)?;
//         let mut id = None;
//         if let Some(params) = params {
//             id = Some(params.id);
//         }
//         let account = Self::from_account_basic(
//             public_key,
//             &mut rpc_data,
//             &reader.root_pda.key,
//             &reader.program_id,
//             // id,
//         )?;
//
//         Ok(account)
//     }
// }