use crate::utils::errors::CustomResponseErrors;
use crate::utils::errors::CustomResponseErrors::ConnectionProblems;
use crate::CustomResponseErrors::InvalidAddress;
use serde::Serialize;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[derive(Debug, Serialize)]
pub struct Wallet {
    address: String,
    tier_1: u64,
    tier_2: u64,
    tier_3: u64,
    tier_4: u64,
    tier_5: u64,
    tier_6: u64,
    tier_7: u64,
}

pub async fn balance(address: &str) -> Result<Wallet, CustomResponseErrors> {
    let t = web3::transports::Http::new(
        "https://eth-mainnet.g.alchemy.com/v2/cBwN5ZXIZkf_C8g5t_aWAloZ9p2RQ7BS",
    );
    let transport = match t {
        Ok(transport) => transport,
        Err(_error) => {
            return Err(ConnectionProblems(String::from(
                "Connection problems to the blockchain",
            )))
        }
    };

    let web3 = web3::Web3::new(transport);

    let address = Address::from_str(address);
    let address = match address {
        Ok(address) => address,
        Err(_error) => return Err(InvalidAddress(String::from("Invalid address"))),
    };

    let contract_address = Address::from_str("0xAA6b31c759e98D38D5a6DDbb4ED58F076183115C").unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../res/mina_abi.json"),
    )
    .unwrap();

    let addresses: Vec<Address> = vec![address; 7];
    let token_ids: Vec<U256> = (1..=7).map(|x: i32| U256::from(x)).collect();

    let o: Result<Vec<U256>, web3::contract::Error> = contract
        .query(
            "balanceOfBatch",
            (addresses, token_ids),
            None,
            Options::default(),
            None,
        )
        .await;

    let balance = o.unwrap();
    let wallet = Wallet {
        address: address.to_string(),
        tier_1: u64::try_from(balance[0]).unwrap(),
        tier_2: u64::try_from(balance[1]).unwrap(),
        tier_3: u64::try_from(balance[2]).unwrap(),
        tier_4: u64::try_from(balance[3]).unwrap(),
        tier_5: u64::try_from(balance[4]).unwrap(),
        tier_6: u64::try_from(balance[5]).unwrap(),
        tier_7: u64::try_from(balance[6]).unwrap(),
    };

    Ok(wallet)
}
