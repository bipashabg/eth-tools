use actix_web::{web, HttpResponse, Responder};
use ethabi::{encode, Token, ParamType};
use ethereum_types::{U256, H160};
use hex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct AbiEncodingRequest {
    pub abi_types: Vec<String>,
    pub parameters: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AbiEncodingResponse {
    pub encoded_output: String,
    pub debug_info: Option<EncodingDebugInfo>,
}

#[derive(Debug, Serialize)]
pub struct EncodingDebugInfo {
    pub parsed_types: Vec<String>,
    pub parsed_values: Vec<String>,
    pub hex_length: usize,
}

#[derive(Debug)]
enum EncodingError {
    TypeParseError(String),
    ParameterParseError(String),
    LengthMismatch,
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::TypeParseError(msg) => write!(f, "Type parsing error: {}", msg),
            EncodingError::ParameterParseError(msg) => write!(f, "Parameter parsing error: {}", msg),
            EncodingError::LengthMismatch => write!(f, "Number of types and parameters must match"),
        }
    }
}

pub async fn encode_abi(request: web::Json<AbiEncodingRequest>) -> impl Responder {
    match encode_parameters(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(format!("Encoding error: {}", e)),
    }
}

fn encode_parameters(request: AbiEncodingRequest) -> Result<AbiEncodingResponse, EncodingError> {
    if request.abi_types.len() != request.parameters.len() {
        return Err(EncodingError::LengthMismatch);
    }

    let param_types: Result<Vec<ParamType>, _> = request.abi_types
        .iter()
        .map(|t| parse_type(t))
        .collect();

    let param_types = param_types.map_err(|e| EncodingError::TypeParseError(e))?;

    let tokens: Result<Vec<Token>, _> = request.parameters
        .iter()
        .zip(param_types.iter())
        .map(|(param, param_type)| parse_parameter(param, param_type))
        .collect();

    let tokens = tokens.map_err(|e| EncodingError::ParameterParseError(e))?;

    let encoded = encode(&tokens);
    let hex_output = format!("0x{}", hex::encode(&encoded));

    let debug_info = EncodingDebugInfo {
        parsed_types: param_types.iter().map(|t| format!("{:?}", t)).collect(),
        parsed_values: tokens.iter().map(|t| format!("{:?}", t)).collect(),
        hex_length: encoded.len(),
    };

    Ok(AbiEncodingResponse {
        encoded_output: hex_output,
        debug_info: Some(debug_info),
    })
}

fn parse_type(type_str: &str) -> Result<ParamType, String> {
    match type_str.trim().to_lowercase().as_str() {
        "address" => Ok(ParamType::Address),
        "uint256" | "uint" => Ok(ParamType::Uint(256)),
        "int256" | "int" => Ok(ParamType::Int(256)),
        "bool" => Ok(ParamType::Bool),
        "string" => Ok(ParamType::String),
        "bytes" => Ok(ParamType::Bytes),
        s if s.starts_with("bytes") => {
            let size: usize = s[5..].parse().map_err(|_| "Invalid bytes size")?;
            if size == 0 || size > 32 {
                return Err("Bytes size must be between 1 and 32".into());
            }
            Ok(ParamType::FixedBytes(size))
        }
        s if s.starts_with("uint") => {
            let size: usize = s[4..].parse().map_err(|_| "Invalid uint size")?;
            if size == 0 || size > 256 || size % 8 != 0 {
                return Err("Invalid uint size - must be multiple of 8 and <= 256".into());
            }
            Ok(ParamType::Uint(size))
        }
        s if s.starts_with("int") => {
            let size: usize = s[3..].parse().map_err(|_| "Invalid int size")?;
            if size == 0 || size > 256 || size % 8 != 0 {
                return Err("Invalid int size - must be multiple of 8 and <= 256".into());
            }
            Ok(ParamType::Int(size))
        }
        _ => Err(format!("Unsupported type: {}", type_str)),
    }
}

fn parse_parameter(param: &str, param_type: &ParamType) -> Result<Token, String> {
    let param = param.trim();
    match param_type {
        ParamType::Address => {
            if !param.starts_with("0x") || param.len() != 42 {
                return Err("Invalid address format - must be 0x followed by 40 hex characters".into());
            }
            Ok(Token::Address(
                H160::from_str(param).map_err(|_| "Invalid address hex format")?
            ))
        }
        ParamType::Uint(_) => {
            Ok(Token::Uint(parse_number(param)?))
        }
        ParamType::Int(_) => {
            Ok(Token::Int(parse_number(param)?))
        }
        ParamType::Bool => {
            match param.to_lowercase().as_str() {
                "true" | "1" => Ok(Token::Bool(true)),
                "false" | "0" => Ok(Token::Bool(false)),
                _ => Err("Invalid boolean value - use true/false or 1/0".into()),
            }
        }
        ParamType::String => {
            let cleaned = param.trim_matches('"').trim_matches('\'');
            Ok(Token::String(cleaned.to_string()))
        }
        ParamType::Bytes => {
            if !param.starts_with("0x") {
                return Err("Bytes must start with 0x prefix".into());
            }
            let bytes = hex::decode(&param[2..])
                .map_err(|_| "Invalid hex string for bytes")?;
            Ok(Token::Bytes(bytes))
        }
        ParamType::FixedBytes(size) => {
            if !param.starts_with("0x") {
                return Err("Fixed bytes must start with 0x prefix".into());
            }
            let bytes = hex::decode(&param[2..])
                .map_err(|_| "Invalid hex string for fixed bytes")?;
            if bytes.len() != *size {
                return Err(format!("Fixed bytes must be exactly {} bytes", size));
            }
            Ok(Token::FixedBytes(bytes))
        }
        _ => Err(format!("Unsupported parameter type: {:?}", param_type)),
    }
}

fn parse_number(param: &str) -> Result<U256, String> {
    if param.starts_with("0x") {
        U256::from_str(param).map_err(|_| "Invalid hex number format".to_string())
    } else {
        U256::from_dec_str(param).map_err(|_| "Invalid decimal number format".to_string())
    }
}