use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use nft_shared::utils::balance_batch::Wallet;
use nft_shared::utils::events::Response;
use nft_shared::utils::owners::QueryResponse;

use nft_shared::utils::errors::CustomResponseErrors;
use nft_shared::utils::errors::CustomResponseErrors::InvalidAddress;

use serde::Deserialize;

#[derive(Deserialize)]
struct CursorQuery {
    cursor: Option<String>,
}

#[derive(Deserialize)]
struct BlockQuery {
    block: Option<u64>,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn balance_batch(address: web::Path<String>) -> Result<Json<Wallet>, CustomResponseErrors> {
    let address = address.into_inner();
    let balance_result = nft_shared::balance(address.as_str()).await;
    let balance = match balance_result {
        Ok(result) => result,
        Err(_e) => return Err(InvalidAddress("Invalid Address".to_string())),
    };

    Ok(web::Json(balance))
}

async fn events(
    block_query: web::Query<BlockQuery>,
) -> Result<Json<Response>, CustomResponseErrors> {
    let block_option = block_query.block;
    let block = match block_option {
        Some(block) => block,
        None => 14799868,
    };
    let events_result = nft_shared::events(block).await;
    let events = match events_result {
        Ok(result) => result,
        Err(_e) => return Err(InvalidAddress("Invalid block".to_string())),
    };

    Ok(web::Json(events))
}

async fn owners(
    cursor_query: web::Query<CursorQuery>,
) -> Result<Json<QueryResponse>, CustomResponseErrors> {
    let cursor_option = cursor_query.cursor.to_owned();
    let cursor = match cursor_option {
        Some(cursor) => cursor,
        None => "".to_string(),
    };
    let owners_result = nft_shared::owners(cursor.as_str()).await;
    let owners = match owners_result {
        Ok(result) => result,
        Err(_e) => return Err(InvalidAddress("Invalid block".to_string())),
    };

    Ok(web::Json(owners))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/balance/{address}", web::get().to(balance_batch))
            .route("/events", web::get().to(events))
            .route("/owners", web::get().to(owners))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
