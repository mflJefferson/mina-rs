mod utils;

// use ethabi::ParamType;
// use ethcontract::prelude::*;
// use ethcontract::web3::signing::Key;
// use ethcontract::web3::types::U64 as eth_U64;
// use futures::TryStreamExt as _;
// use hex_literal::hex;
// use serde_json::*;
// use std::borrow::Borrow;
// use std::str::FromStr;
// use std::time;
// use web3::contract::Contract;

// use web3::futures::FutureExt;
use crate::utils::errors::CustomResponseErrors;
use utils::{balance_batch, events};
// use web3::transports::Http;
// use web3::types::{Address, BlockNumber as web3_BlockNumber, FilterBuilder, U64};

// ethcontract::contract!(
//     "nft_shared/src/res/MinaNft.json",
//     contract = MinaNft,
//     deployments {
//         1 => "0xAA6b31c759e98D38D5a6DDbb4ED58F076183115C",
//         4 => "0xec966AaaD6D468faDF7E4148b9222c6aee8bB767"
//     }
// );

// #[tokio::main]
// pub async fn not_main() {
// let web3_url = "https://eth-rinkeby.alchemyapi.io/v2/B9gXQzuzwdGzwgINlmYmvQGD7Gfr6Sbi";
//
// let http = Http::new(&web3_url).expect("transport failed");
// let web3 = Web3::new(http);
//
// let instance = MinaNft::deployed(&web3)
//     .await
//     .expect("locating deployed contract failed");
// let symbol = instance.owner().call().await.expect("get symbol failed");

// println!("a : {:?} ", instance);
// println!("b : {:?} ", symbol);
//
// println!("Retrieving all past events (this could take a while)...");
// let from_block = "0xa31173".into();
// let genesis = BlockNumber::Number("0xa31173".into());
// assert_eq!(format!("{:x}", 10686835), "a31173");
// let from_block = format!("0x{:x}", 10686835);
// println!("genesis {:?}", genesis);
// dbg!(instance);
// let event_history_stream = instance
//     .all_events()
//     .from_block(from_block)
//     .to_block(BlockNumber::Latest)
//     .query()
//     .await
//     .expect("Couldn't retrieve event history");

// println!("{:?}", &event_history_stream[2].data)
// println!("genesis {:?}", event_history_stream);

// let event_history_vec = event_history_stream
//     .try_collect::<Vec<_>>()
//     .await
//     .expect("Couldn't parse event");
// println!(
//     "Total number of events emitted by OWL token {:}",
//     event_history_vec.len()
// );
// }

#[tokio::main]
pub async fn events(block: u64) -> Result<String, CustomResponseErrors> {
    events::events(block).await
}

#[tokio::main]
pub async fn balance(address: &str) -> Result<String, CustomResponseErrors> {
    balance_batch::balance(address).await
}
