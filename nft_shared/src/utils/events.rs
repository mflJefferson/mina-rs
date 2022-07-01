use crate::CustomResponseErrors;
use crate::CustomResponseErrors::ConnectionProblems;
use ethabi::ParamType;
use hex_literal::hex;
use serde::Serialize;
use std::ops::Index;
use std::str::FromStr;
use web3::contract::Contract;
use web3::transports::Http;
use web3::types::{Address, BlockNumber as web3_BlockNumber, FilterBuilder, U64};

#[derive(Debug, Serialize)]
struct Event {
    transaction_hash: String,
    log_index: u32,
    block_number: u64,
    from: String,
    to: String,
    tier_1: u32,
    tier_2: u32,
    tier_3: u32,
    tier_4: u32,
    tier_5: u32,
    tier_6: u32,
    tier_7: u32,
}

impl Default for Event {
    fn default() -> Event {
        Event {
            transaction_hash: "".to_string(),
            log_index: 0,
            block_number: 0,
            from: "".to_string(),
            to: "".to_string(),
            tier_1: 0,
            tier_2: 0,
            tier_3: 0,
            tier_4: 0,
            tier_5: 0,
            tier_6: 0,
            tier_7: 0,
        }
    }
}

impl Event {
    fn tier_value(&mut self, tier: u32, value: u32) {
        match tier {
            1 => self.tier_1 = value,
            2 => self.tier_2 = value,
            3 => self.tier_3 = value,
            4 => self.tier_4 = value,
            5 => self.tier_5 = value,
            6 => self.tier_6 = value,
            7 => self.tier_7 = value,
            _ => {}
        }
    }
}

#[derive(Debug, Serialize)]
struct Response {
    events: Vec<Event>,
}

pub async fn events(block: u64) -> Result<String, CustomResponseErrors> {
    let t = web3::transports::Http::new(
        "https://eth-rinkeby.alchemyapi.io/v2/B9gXQzuzwdGzwgINlmYmvQGD7Gfr6Sbi",
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

    let address = Address::from_str("0xec966AaaD6D468faDF7E4148b9222c6aee8bB767").unwrap();
    let contract =
        Contract::from_json(web3.eth(), address, include_bytes!("../res/mina_abi.json")).unwrap();

    let block_number: U64 = format!("{:x}", block).parse().unwrap();
    let from_block = web3_BlockNumber::Number(block_number);
    let latest = web3_BlockNumber::Latest;
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .from_block(from_block)
        .to_block(web3_BlockNumber::Latest)
        .topics(
            Some(vec![
                hex!("c3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62").into(),
                hex!("4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb").into(),
            ]),
            None,
            None,
            None,
        )
        .build();
    let l = web3.eth().logs(filter).await;

    let logs = match l {
        Ok(log) => log,
        Err(e) => {
            return Err(ConnectionProblems(String::from("Error getting event logs")));
        }
    };
    let mut response = Response { events: vec![] };
    for log in logs.iter() {
        let topic_1 = "0xc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62"
            .parse()
            .unwrap();

        let mut event = "TransferSingle";
        if log.topics[0] != topic_1 {
            event = "TransferBatch";
        }

        let decoded_data_op = contract
            .abi()
            .event(event)
            .unwrap()
            .parse_log(ethabi::RawLog {
                topics: log.topics.clone(),
                data: log.data.0.clone(),
            });

        let decoded_data = match decoded_data_op {
            Ok(decoded_data) => decoded_data,
            Err(e) => {
                return Err(ConnectionProblems(String::from(
                    "Error decoding event data",
                )));
            }
        };
        let decoded_vec = decoded_data
            .params
            .into_iter()
            .map(|x| x.value)
            .collect::<Vec<_>>();

        let mut event = Event {
            ..Default::default()
        };

        event.transaction_hash = log.transaction_hash.unwrap().to_string();
        event.block_number = log.block_number.unwrap().as_u64();
        event.log_index = log.log_index.unwrap().as_u32();
        event.from = format!("0x{}", decoded_vec[1].to_string());
        event.to = format!("0x{}", decoded_vec[2].to_string());

        let is_integer = decoded_vec[3].type_check(&ParamType::Uint(256));
        if is_integer {
            let tier = decoded_vec[3].to_owned();
            let value = decoded_vec[4].to_owned();
            event.tier_value(
                tier.into_uint().unwrap().as_u32(),
                value.into_uint().unwrap().as_u32(),
            );
        } else {
            let tiers = decoded_vec[3].to_owned().into_array().unwrap().to_owned();
            let values = decoded_vec[4].to_owned().into_array().unwrap().to_owned();
            for (index, tier) in tiers.iter().enumerate() {
                let t = tier.to_owned();
                let v = values.index(index).to_owned();
                event.tier_value(
                    t.into_uint().unwrap().as_u32(),
                    v.into_uint().unwrap().as_u32(),
                );
            }
        }
        response.events.push(event);
    }

    Ok(serde_json::to_string(&response).unwrap())
}
