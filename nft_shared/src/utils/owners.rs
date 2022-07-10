use crate::CustomResponseErrors;
use crate::CustomResponseErrors::ConnectionProblems;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    token_address: String,
    token_id: String,
    contract_type: String,
    owner_of: String,
    block_number: String,
    block_number_minted: String,
    token_uri: String,
    metadata: String,
    amount: String,
    name: Option<String>,
    symbol: Option<String>,
    token_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    total: u64,
    page: u64,
    page_size: u64,
    cursor: Option<String>,
    result: Vec<QueryResult>,
    status: Option<String>,
}

pub async fn nft_owners(cursor: &str) -> Result<QueryResponse, CustomResponseErrors> {
    let mut url = dotenv!("MORALIS_URL").to_owned();
    url.push_str(cursor);
    let resp_result = reqwest::Client::new()
        .get(url)
        .header("x-api-key", dotenv!("MORALIS_KEY"))
        .send()
        .await;
    let resp = match resp_result {
        Ok(resp_result) => resp_result,
        Err(e) => {
            return Err(ConnectionProblems(String::from(e.to_string())));
        }
    };
    let query_response_result = resp.json::<QueryResponse>().await;
    let query_response = match query_response_result {
        Ok(result) => result,
        Err(e) => {
            return Err(ConnectionProblems(String::from(e.to_string())));
        }
    };
    Ok(query_response)
}
