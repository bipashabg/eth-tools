use reqwest::Client;
use serde_json::Value;

pub async fn get_block_timestamp(block_number: u64, chain_id: u64) -> Result<u64, String> {
    let client = Client::new();
    let api_url = match chain_id {
        1 => "https://mainnet.infura.io/v3/eabf840bc6154619be65f9db30cfb7e9 ", // Mainnet   https://rpc.sepolia.org    https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID
        4 => "https://rinkeby.infura.io/v3/eabf840bc6154619be65f9db30cfb7e9", // Rinkeby Testnet
        _ => return Err("Unsupported chain id".to_string()),
    };

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [format!("0x{:x}", block_number), false],
        "id": 1,
    });

    let res = client
        .post(api_url)
        .json(&body)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let json: Value = res.json().await.map_err(|err| err.to_string())?;

    if let Some(timestamp_str) = json["result"]["timestamp"].as_str() {
        let timestamp = u64::from_str_radix(timestamp_str.trim_start_matches("0x"), 16)
            .map_err(|err| err.to_string())?;
        Ok(timestamp)
    } else {
        Err("Failed to retrieve block timestamp".to_string())
    }
}
