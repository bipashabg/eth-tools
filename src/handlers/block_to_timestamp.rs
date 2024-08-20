use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::blockchain::get_block_timestamp;

#[derive(Deserialize)]
pub struct BlockRequest {
    block_number: u64,
    chain_id: u64,
}

#[derive(Serialize)]
pub struct BlockResponse {
    timestamp: u64,
}

pub async fn block_to_timestamp(req: web::Json<BlockRequest>) -> HttpResponse {
    match get_block_timestamp(req.block_number, req.chain_id).await {
        Ok(timestamp) => HttpResponse::Ok().json(BlockResponse { timestamp }),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
