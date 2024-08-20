
use serde_json::{Value,json};
use regex::Regex;


pub fn abi_to_solidity(abi: &str) -> Result<String, String> {
    let abi: serde_json::Value = serde_json::from_str(abi).map_err(|e| e.to_string())?;

    let mut solidity_code = String::from("pragma solidity ^0.8.0;\n\ncontract Generated {\n");

    if let Some(abi_items) = abi.as_array() {
        for item in abi_items {
            match item["type"].as_str().unwrap_or("") {
                "function" => {
                    let name = item["name"].as_str().unwrap_or("");
                    let mut inputs = vec![];
                    if let Some(input_arr) = item["inputs"].as_array() {
                        for input in input_arr {
                            let input_type = input["type"].as_str().unwrap_or("");
                            let input_name = input["name"].as_str().unwrap_or("");
                            inputs.push(format!("{} {}", input_type, input_name));
                        }
                    }
                    let inputs_str = inputs.join(", ");
                    
                    let state_mutability = item["stateMutability"].as_str().unwrap_or("");
                    let visibility = item["visibility"].as_str().unwrap_or("public");

                    let payable_str = if state_mutability == "payable" {
                        " payable"
                    } else {
                        ""
                    };

                    let mut output_types = vec![];
                    if let Some(output_arr) = item["outputs"].as_array() {
                        for output in output_arr {
                            output_types.push(output["type"].as_str().unwrap_or(""));
                        }
                    }
                    let output_str = if !output_types.is_empty() {
                        format!(" returns ({})", output_types.join(", "))
                    } else {
                        "".to_string()
                    };

                    solidity_code.push_str(&format!(
                        "    function {}({}) {}{}{} {};\n",
                        name,
                        inputs_str,
                        visibility,
                        payable_str,
                        state_mutability,
                        output_str
                    ));
                },
                "constructor" => {
                    let mut inputs = vec![];
                    if let Some(input_arr) = item["inputs"].as_array() {
                        for input in input_arr {
                            let input_type = input["type"].as_str().unwrap_or("");
                            let input_name = input["name"].as_str().unwrap_or("");
                            inputs.push(format!("{} {}", input_type, input_name));
                        }
                    }
                    let inputs_str = inputs.join(", ");
                    
                    let state_mutability = item["stateMutability"].as_str().unwrap_or("");

                    let payable_str = if state_mutability == "payable" {
                        " payable"
                    } else {
                        ""
                    };

                    solidity_code.push_str(&format!(
                        "    constructor({}){} {}\n",
                        inputs_str,
                        payable_str,
                        state_mutability
                    ));
                },
                "event" => {
                    let name = item["name"].as_str().unwrap_or("");
                    let mut inputs = vec![];
                    if let Some(input_arr) = item["inputs"].as_array() {
                        for input in input_arr {
                            let input_type = input["type"].as_str().unwrap_or("");
                            let input_name = input["name"].as_str().unwrap_or("");
                            let indexed = if input["indexed"].as_bool().unwrap_or(false) {
                                " indexed"
                            } else {
                                ""
                            };
                            inputs.push(format!("{}{} {}", input_type, indexed, input_name));
                        }
                    }
                    let inputs_str = inputs.join(", ");
                    let anonymous = if item["anonymous"].as_bool().unwrap_or(false) {
                        " anonymous"
                    } else {
                        ""
                    };

                    solidity_code.push_str(&format!(
                        "    event {}({}){};\n",
                        name,
                        inputs_str,
                        anonymous
                    ));
                },
                "fallback" => {
                    let state_mutability = item["stateMutability"].as_str().unwrap_or("");

                    let payable_str = if state_mutability == "payable" {
                        " payable"
                    } else {
                        ""
                    };

                    solidity_code.push_str(&format!(
                        "    fallback() external{} {}\n",
                        payable_str,
                        state_mutability
                    ));
                },
                "receive" => {
                    solidity_code.push_str("    receive() external payable {}\n");
                },
                "error" => {
                    let name = item["name"].as_str().unwrap_or("");
                    let mut inputs = vec![];
                    if let Some(input_arr) = item["inputs"].as_array() {
                        for input in input_arr {
                            let input_type = input["type"].as_str().unwrap_or("");
                            let input_name = input["name"].as_str().unwrap_or("");
                            inputs.push(format!("{} {}", input_type, input_name));
                        }
                    }
                    let inputs_str = inputs.join(", ");

                    solidity_code.push_str(&format!(
                        "    error {}({});\n",
                        name,
                        inputs_str
                    ));
                },
                _ => {},
            }
        }
    }

    solidity_code.push_str("}\n");
    Ok(solidity_code)
}


pub fn solidity_to_abi(solidity_code: &str) -> Result<String, String> {
    let mut abi = vec![];

    
    let function_regex = Regex::new(r"function\s+(\w+)\s*\(([^)]*)\)\s*(returns\s*\(([^)]*)\))?\s*(payable|view|pure)?").map_err(|e| e.to_string())?;
    let event_regex = Regex::new(r"event\s+(\w+)\s*\(([^)]*)\)").map_err(|e| e.to_string())?;
    let constructor_regex = Regex::new(r"constructor\s*\(([^)]*)\)\s*(payable)?").map_err(|e| e.to_string())?;
    let fallback_regex = Regex::new(r"fallback\s*\(\)\s*(payable)?").map_err(|e| e.to_string())?;
    let receive_regex = Regex::new(r"receive\s*\(\)\s*(payable)?").map_err(|e| e.to_string())?;
    let error_regex = Regex::new(r"error\s+(\w+)\s*\(([^)]*)\)").map_err(|e| e.to_string())?;

   
    for caps in function_regex.captures_iter(solidity_code) {
        let name = &caps[1];
        let inputs = &caps[2];
        let outputs = caps.get(4).map_or("", |m| m.as_str());
        let state_mutability = caps.get(5).map_or("nonpayable", |m| m.as_str());

        let inputs_vec: Vec<serde_json::Value> = inputs
            .split(',')
            .filter_map(|input| {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(json!({
                        "name": parts[1],
                        "type": parts[0],
                    }))
                } else {
                    None
                }
            })
            .collect();

        let outputs_vec: Vec<serde_json::Value> = if !outputs.is_empty() {
            outputs
                .split(',')
                .filter_map(|output| {
                    let parts: Vec<&str> = output.trim().split_whitespace().collect();
                    if parts.len() >= 1 {
                        Some(json!({
                            "name": "",
                            "type": parts[0],
                        }))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        };

        let mut function_json = json!({
            "inputs": inputs_vec,
            "name": name,
            "outputs": outputs_vec,
            "stateMutability": state_mutability,
            "type": "function"
        });

        
        if state_mutability == "view" || state_mutability == "pure" {
            function_json["constant"] = json!(true);
        }

        abi.push(function_json);
    }

   
    for caps in event_regex.captures_iter(solidity_code) {
        let name = &caps[1];
        let inputs = &caps[2];

        let inputs_vec: Vec<serde_json::Value> = inputs
            .split(',')
            .filter_map(|input| {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(json!({
                        "name": parts[1],
                        "type": parts[0],
                        "indexed": false 
                    }))
                } else {
                    None
                }
            })
            .collect();

        abi.push(json!({
            "anonymous": false,
            "inputs": inputs_vec,
            "name": name,
            "type": "event"
        }));
    }

    
    for caps in constructor_regex.captures_iter(solidity_code) {
        let inputs = &caps[1];
        let state_mutability = if caps.get(2).is_some() { "payable" } else { "nonpayable" };

        let inputs_vec: Vec<serde_json::Value> = inputs
            .split(',')
            .filter_map(|input| {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(json!({
                        "name": parts[1],
                        "type": parts[0],
                    }))
                } else {
                    None
                }
            })
            .collect();

        abi.push(json!({
            "inputs": inputs_vec,
            "name": "",
            "outputs": [],
            "stateMutability": state_mutability,
            "type": "constructor"
        }));
    }

    
    for caps in fallback_regex.captures_iter(solidity_code) {
        let state_mutability = if caps.get(1).is_some() { "payable" } else { "nonpayable" };

        abi.push(json!({
            "inputs": [],
            "name": "",
            "outputs": [],
            "stateMutability": state_mutability,
            "type": "fallback"
        }));
    }

    
    for caps in receive_regex.captures_iter(solidity_code) {
        let state_mutability = if caps.get(1).is_some() { "payable" } else { "nonpayable" };

        abi.push(json!({
            "inputs": [],
            "name": "",
            "outputs": [],
            "stateMutability": state_mutability,
            "type": "receive"
        }));
    }

    
    for caps in error_regex.captures_iter(solidity_code) {
        let name = &caps[1];
        let inputs = &caps[2];

        let inputs_vec: Vec<serde_json::Value> = inputs
            .split(',')
            .filter_map(|input| {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(json!({
                        "name": parts[1],
                        "type": parts[0],
                    }))
                } else {
                    None
                }
            })
            .collect();

        abi.push(json!({
            "inputs": inputs_vec,
            "name": name,
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "error"
        }));
    }

    serde_json::to_string(&abi).map_err(|e| e.to_string())
}

