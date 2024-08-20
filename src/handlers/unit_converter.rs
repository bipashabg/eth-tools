use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::conversions;

#[derive(Deserialize)]
pub struct ConversionRequest {
    value: String,
    conversion_type: String,
}

#[derive(Serialize)]
pub struct ConversionResponse {
    result: String,
}

pub async fn unit_converter(req: web::Json<ConversionRequest>) -> HttpResponse {
    let result = match req.conversion_type.as_str() {
        "hex_to_decimal" => conversions::hex_to_decimal(&req.value)
            .map(|res| res.to_string())
            .unwrap_or_else(|_| "Invalid hex".to_string()),
        "decimal_to_hex" => conversions::decimal_to_hex(req.value.parse().unwrap_or(0)),
        "eth_to_wei" => conversions::eth_to_wei(req.value.parse().unwrap_or(0.0)).to_string(),
        "wei_to_eth" => conversions::wei_to_eth(req.value.parse().unwrap_or(0)).to_string(),
        _ => "Unsupported conversion type".to_string(),
    };

    HttpResponse::Ok().json(ConversionResponse { result })
}
