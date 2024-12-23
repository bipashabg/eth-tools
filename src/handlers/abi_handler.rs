use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct AbiRequest {
    pub abi: String,
}

#[derive(Debug, Deserialize)]
pub struct SolidityRequest {
    pub solidity_code: String,
}

#[derive(Debug, Serialize)]
pub struct FormattedContractFunction {
    pub name: String,
    pub inputs: Vec<FormattedParameter>,
    pub outputs: Vec<FormattedParameter>,
    pub state_mutability: String,
    pub function_type: String,
}

#[derive(Debug, Serialize)]
pub struct FormattedParameter {
    pub name: String,
    pub parameter_type: String,
    pub components: Option<Vec<FormattedParameter>>,
}

#[derive(Debug, Serialize)]
pub struct FormattedResponse {
    pub contract_name: String,
    pub version: String,
    pub functions: Vec<FormattedContractFunction>,
    pub events: Vec<FormattedEvent>,
    pub constructor: Option<FormattedContractFunction>,
}

#[derive(Debug, Serialize)]
pub struct FormattedEvent {
    pub name: String,
    pub inputs: Vec<FormattedEventParameter>,
    pub anonymous: bool,
}

#[derive(Debug, Serialize)]
pub struct FormattedEventParameter {
    pub name: String,
    pub parameter_type: String,
    pub indexed: bool,
}

pub async fn convert_abi_to_solidity(request: web::Json<AbiRequest>) -> impl Responder {
    match parse_and_format_abi(&request.abi) {
        Ok(formatted_response) => {
            HttpResponse::Ok().json(formatted_response)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

fn parse_and_format_abi(abi_str: &str) -> Result<FormattedResponse, serde_json::Error> {
    let abi: serde_json::Value = serde_json::from_str(abi_str)?;
    
    let mut response = FormattedResponse {
        contract_name: "Generated".to_string(),
        version: "^0.8.0".to_string(),
        functions: Vec::new(),
        events: Vec::new(),
        constructor: None,
    };

    if let Some(array) = abi.as_array() {
        for item in array {
            match item["type"].as_str() {
                Some("function") => {
                    let function = parse_function(item)?;
                    response.functions.push(function);
                },
                Some("event") => {
                    let event = parse_event(item)?;
                    response.events.push(event);
                },
                Some("constructor") => {
                    let constructor = parse_function(item)?;
                    response.constructor = Some(constructor);
                },
                _ => continue,
            }
        }
    }

    response.functions.sort_by(|a, b| a.name.cmp(&b.name));
    response.events.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(response)
}

fn parse_function(item: &serde_json::Value) -> Result<FormattedContractFunction, serde_json::Error> {
    Ok(FormattedContractFunction {
        name: item["name"].as_str().unwrap_or("").to_string(),
        inputs: parse_parameters(&item["inputs"])?,
        outputs: parse_parameters(&item["outputs"])?,
        state_mutability: item["stateMutability"].as_str().unwrap_or("nonpayable").to_string(),
        function_type: item["type"].as_str().unwrap_or("function").to_string(),
    })
}

fn parse_event(item: &serde_json::Value) -> Result<FormattedEvent, serde_json::Error> {
    Ok(FormattedEvent {
        name: item["name"].as_str().unwrap_or("").to_string(),
        inputs: parse_event_parameters(&item["inputs"])?,
        anonymous: item["anonymous"].as_bool().unwrap_or(false),
    })
}

fn parse_parameters(params: &serde_json::Value) -> Result<Vec<FormattedParameter>, serde_json::Error> {
    let mut formatted_params = Vec::new();
    
    if let Some(params_array) = params.as_array() {
        for param in params_array {
            formatted_params.push(FormattedParameter {
                name: param["name"].as_str().unwrap_or("").to_string(),
                parameter_type: param["type"].as_str().unwrap_or("").to_string(),
                components: if param["components"].is_array() {
                    Some(parse_parameters(&param["components"])?)
                } else {
                    None
                },
            });
        }
    }
    
    Ok(formatted_params)
}

fn parse_event_parameters(params: &serde_json::Value) -> Result<Vec<FormattedEventParameter>, serde_json::Error> {
    let mut formatted_params = Vec::new();
    
    if let Some(params_array) = params.as_array() {
        for param in params_array {
            formatted_params.push(FormattedEventParameter {
                name: param["name"].as_str().unwrap_or("").to_string(),
                parameter_type: param["type"].as_str().unwrap_or("").to_string(),
                indexed: param["indexed"].as_bool().unwrap_or(false),
            });
        }
    }
    
    Ok(formatted_params)
}

