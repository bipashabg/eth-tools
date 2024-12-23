use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ConversionRequest {
    value: String,
    from_unit: String,
}

#[derive(Serialize)]
pub struct ConversionResponse {
    wei: String,
    gwei: String,
    eth: String,
}

const WEI_TO_GWEI: u32 = 9;  
const WEI_TO_ETH: u32 = 18;  

pub async fn unit_converter(req: web::Json<ConversionRequest>) -> HttpResponse {
    let value = match Decimal::from_str(&req.value) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().json("Invalid number format"),
    };

    let (wei, gwei, eth) = match req.from_unit.as_str() {
        "wei" => convert_from_wei(value),
        "gwei" => convert_from_gwei(value),
        "eth" => convert_from_eth(value),
        _ => return HttpResponse::BadRequest().json("Unsupported unit type"),
    };

    HttpResponse::Ok().json(ConversionResponse {
        wei,
        gwei,
        eth,
    })
}

fn convert_from_wei(wei_value: Decimal) -> (String, String, String) {
    let gwei = wei_value / Decimal::from(10i64.pow(WEI_TO_GWEI));
    let eth = wei_value / Decimal::from(10i64.pow(WEI_TO_ETH));

    (
        wei_value.to_string(),
        gwei.to_string(),
        eth.to_string(),
    )
}

fn convert_from_gwei(gwei_value: Decimal) -> (String, String, String) {
    let wei = gwei_value * Decimal::from(10i64.pow(WEI_TO_GWEI));
    let eth = gwei_value / Decimal::from(10i64.pow(WEI_TO_GWEI));

    (
        wei.to_string(),
        gwei_value.to_string(),
        eth.to_string(),
    )
}

fn convert_from_eth(eth_value: Decimal) -> (String, String, String) {
    let wei = eth_value * Decimal::from(10i64.pow(WEI_TO_ETH));
    let gwei = eth_value * Decimal::from(10i64.pow(WEI_TO_ETH - WEI_TO_GWEI));

    (
        wei.to_string(),
        gwei.to_string(),
        eth_value.to_string(),
    )
}