pub use colored::Colorize;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    pub mainnet: AddressInfo,
    pub testnet: AddressInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    #[serde(rename = "STACKS")]
    pub stacks: String,
    #[serde(rename = "BTC")]
    pub btc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub mnemonic: String,
    pub private_key: String,
    pub stx_address: String,
    pub btc_address: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Keychain {
    pub mnemonic: String,
    #[serde(rename = "keyInfo")]
    pub key_info: KeyInfo,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct KeyInfo {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub address: String,
    #[serde(rename = "btcAddress")]
    pub btc_address: String,
    pub wif: String,
    pub index: u32,
}

#[derive(Deserialize, Debug)]
pub struct AccountInfo {
    pub balance: String,
    pub nonce: u64,
    #[serde(default)] // Optional field
    pub proof: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AccountBalance {
    pub balance: String,
}

#[macro_export]
macro_rules! error_m {
    ($name:expr) => {
        colored::Colorize::red(format!("ERR: {}", $name).as_str())
    };
}

#[macro_export]
macro_rules! warn_m {
    ($name:expr) => {
        colored::Colorize::yellow(format!("WARN: {}!", $name).as_str())
    };
}

#[macro_export]
macro_rules! info_m {
    ($name:expr) => {
        colored::Colorize::green(format!("INFO: {}", $name).as_str())
    };
}
