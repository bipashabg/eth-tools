use actix_web::web;
use crate::handlers::{
    unit_converter,
    block_to_timestamp,
    abi_handler,
    abi_encoder,
    string_converter
};
use crate::txn_decoder::finality_decode;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/convert")
            .route(web::post().to(unit_converter::unit_converter))
    )
    .service(
        web::resource("/block-to-timestamp")
            .route(web::post().to(block_to_timestamp::block_to_timestamp))
    )
    .service(
        web::resource("/convert-abi-to-solidity")
            .route(web::post().to(abi_handler::convert_abi_to_solidity))
    )
    .service(
        web::resource("/convert-solidity-to-abi")
            .route(web::post().to(abi_encoder::encode_abi))
    )
    .service(
        web::resource("/decode-transaction")
            .route(web::post().to(finality_decode::decode))
    )
    .service(
        web::resource("/encode-abi")
            .route(web::post().to(abi_encoder::encode_abi))
    )
    .service(
        web::resource("/convert-string")
            .route(web::post().to(string_converter::convert_string))
    );
}