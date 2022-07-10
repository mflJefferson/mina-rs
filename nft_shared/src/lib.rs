pub mod utils;

#[macro_use]
extern crate dotenv_codegen;

use crate::balance_batch::Wallet;
use crate::events::Response;
use crate::owners::QueryResponse;
use crate::utils::errors::CustomResponseErrors;
use utils::{balance_batch, events, owners};

pub async fn events(block: u64) -> Result<Response, CustomResponseErrors> {
    events::events(block).await
}

pub async fn balance(address: &str) -> Result<Wallet, CustomResponseErrors> {
    balance_batch::balance(address).await
}

pub async fn owners(cursor: &str) -> Result<QueryResponse, CustomResponseErrors> {
    owners::nft_owners(cursor).await
}
