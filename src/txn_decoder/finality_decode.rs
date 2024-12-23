use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::txn_decoder::{
    function_decoder::{function_decoder_with_abi, function_decoder_without_abi},
    transaction_decoder,
};

// let txn = "0x02f8ea05827cc5425a830162e894758502f01b613287731c0788f1fe4d5c4c19b79b80b8849aaab648094e2b6c42ee984bdaf54f32cda9628d8b345f6d02d05b7ec922999ad5f1eafb000000000000000000000000000000000000000000000000000000000020d3489b3732e86fde103e1bf5efd4a317e2aa93c7deb97b8bae232c7e957064370f7c00000000000000000000000000000000000000000000000000000000009fb68bc080a08e6eb39efd60bb1bc18c137071b7cd2b41ea5f58316984f15e4e4d1100c7b5d1a0523dc6b210cdd705478293de03aabebc8e788caa13de5a3293b5651f88b80a22";
    // let abi_json = r#"[{"inputs":[{"internalType":"uint256","name":"_submissionInterval","type":"uint256"},{"internalType":"uint256","name":"_l2BlockTime","type":"uint256"},{"internalType":"uint256","name":"_finalizationPeriodSeconds","type":"uint256"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint8","name":"version","type":"uint8"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"bytes32","name":"outputRoot","type":"bytes32"},{"indexed":true,"internalType":"uint256","name":"l2OutputIndex","type":"uint256"},{"indexed":true,"internalType":"uint256","name":"l2BlockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"l1Timestamp","type":"uint256"}],"name":"OutputProposed","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"uint256","name":"prevNextOutputIndex","type":"uint256"},{"indexed":true,"internalType":"uint256","name":"newNextOutputIndex","type":"uint256"}],"name":"OutputsDeleted","type":"event"},{"inputs":[],"name":"CHALLENGER","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"FINALIZATION_PERIOD_SECONDS","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"L2_BLOCK_TIME","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"PROPOSER","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"SUBMISSION_INTERVAL","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"challenger","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_l2BlockNumber","type":"uint256"}],"name":"computeL2Timestamp","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_l2OutputIndex","type":"uint256"}],"name":"deleteL2Outputs","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"finalizationPeriodSeconds","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_l2OutputIndex","type":"uint256"}],"name":"getL2Output","outputs":[{"components":[{"internalType":"bytes32","name":"outputRoot","type":"bytes32"},{"internalType":"uint128","name":"timestamp","type":"uint128"},{"internalType":"uint128","name":"l2BlockNumber","type":"uint128"}],"internalType":"struct Types.OutputProposal","name":"","type":"tuple"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_l2BlockNumber","type":"uint256"}],"name":"getL2OutputAfter","outputs":[{"components":[{"internalType":"bytes32","name":"outputRoot","type":"bytes32"},{"internalType":"uint128","name":"timestamp","type":"uint128"},{"internalType":"uint128","name":"l2BlockNumber","type":"uint128"}],"internalType":"struct Types.OutputProposal","name":"","type":"tuple"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_l2BlockNumber","type":"uint256"}],"name":"getL2OutputIndexAfter","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"_startingBlockNumber","type":"uint256"},{"internalType":"uint256","name":"_startingTimestamp","type":"uint256"},{"internalType":"address","name":"_proposer","type":"address"},{"internalType":"address","name":"_challenger","type":"address"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"l2BlockTime","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"latestBlockNumber","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"latestOutputIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"nextBlockNumber","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"nextOutputIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes32","name":"_outputRoot","type":"bytes32"},{"internalType":"uint256","name":"_l2BlockNumber","type":"uint256"},{"internalType":"bytes32","name":"_l1BlockHash","type":"bytes32"},{"internalType":"uint256","name":"_l1BlockNumber","type":"uint256"}],"name":"proposeL2Output","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"proposer","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"startingBlockNumber","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"startingTimestamp","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"submissionInterval","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"version","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"}]"#;


pub async fn decode(request_body: web::Json<TransactionRequest>) -> impl Responder {
    let (calldata, tx) = transaction_decoder::txn_decoder(request_body.txn.as_str());
    let data: Vec<u8> = match calldata {
        Ok(calldata) => calldata,
        Err(e) => return HttpResponse::BadRequest().json(json!({
            "txn": null,
            "function_called": null,
            "parameters": null,
            "err": e.to_string(),
        })),
    };

    let result = if let Some(abi_json) = &request_body.abi_json {
        // With ABI
        let decoder = match function_decoder_with_abi::CalldataDecoder::new(abi_json) {
            Ok(decoder) => decoder,
            Err(e) => return HttpResponse::BadRequest().json(json!({
                "txn": tx,
                "function_called": null,
                "parameters": null,
                "err": format!("Failed to create decoder: {}", e),
            })),
        };

        match decoder.decode_calldata(hex::encode(data.as_ref() as &Vec<u8>).as_str()) {
            Ok((function_name, decoded_params)) => {
                let decoded_params_string: Vec<String> = decoded_params
                    .iter()
                    .map(|param| function_decoder_with_abi::token_to_string(param))
                    .collect();

                json!({
                    "txn": tx,
                    "function_called": function_name,
                    "parameters": decoded_params_string,
                    "err": null,
                })
            }
            Err(e) => json!({
                "txn": tx,
                "function_called": null,
                "parameters": null,
                "err": format!("Failed to decode calldata: {}", e),
            }),
        }
    } else {
        // Without ABI
        match function_decoder_without_abi::final_result_from_calldata(&hex::encode(data)).await {
            (Ok(()), function_name, decoded_params_string) => json!({
                "txn": tx,
                "function_called": function_name,
                "parameters": decoded_params_string,
                "err": null,
            }),
            (Err(e), _, _) => json!({
                "txn": tx,
                "function_called": null,
                "parameters": null,
                "err": format!("Failed to decode function: {}", e),
            }),
        }
    };

    HttpResponse::Ok().json(result)
}
#[derive(Deserialize)]
pub struct TransactionRequest {
    pub txn: String,
    pub abi_json: Option<String>,
}

#[derive(Serialize)]
pub struct TransactionDecodeResult {
    pub txn: Option<transaction_decoder::Transaction>,
    pub function_called: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub err: Option<String>,
}