
use actix_web::{web, HttpResponse, Responder};
use crate::utils::sol_utils::{abi_to_solidity,solidity_to_abi};

pub async fn convert_abi_to_solidity(
    request_body: web::Json<AbiRequest>,
) -> impl Responder {
    let abi = &request_body.abi;
    match abi_to_solidity(abi) {
        Ok(solidity_code) => HttpResponse::Ok().body(solidity_code),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn convert_solidity_to_abi(
    request_body: web::Json<SolidityRequest>,
) -> impl Responder {
    let solidity_code = &request_body.solidity_code;
    match solidity_to_abi(solidity_code) {
        Ok(abi) => HttpResponse::Ok().body(abi),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[derive(serde::Deserialize)]
pub struct SolidityRequest {
    pub solidity_code: String,
}

#[derive(serde::Deserialize)]
pub struct AbiRequest {
    pub abi: String,
}


