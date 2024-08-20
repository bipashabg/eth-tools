pub fn hex_to_decimal(hex: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(hex.trim_start_matches("0x"), 16)
}

pub fn decimal_to_hex(decimal: u64) -> String {
    format!("0x{:x}", decimal)
}

pub fn eth_to_wei(eth: f64) -> u128 {
    (eth * 1e18) as u128
}

pub fn wei_to_eth(wei: u128) -> f64 {
    (wei as f64) / 1e18
}


