use crate::consts::AddressData;
use crate::error_m;

use bip39::Mnemonic;
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::Network;
use std::process::Command;
use std::str::FromStr;

pub fn convert_btc_stx_adress(btc_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "stx convert_address {} | jq > stx_address.json",
            btc_address
        ))
        .status()?;

    if !status.success() {
        eprintln!("{}", error_m!("Failed to execute `stx convert_address`"));
        return Err("Failed to execute command".into());
    }

    Ok(())
}

pub fn derive_private_key_from_mnemonic(mnemonic_phrase: &str,) -> Result<String, Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::from_str(mnemonic_phrase)?;
    let seed = mnemonic.to_seed("");
    let secp = Secp256k1::new();

    let master_key = Xpriv::new_master(Network::Bitcoin, &seed)?;
    let path = DerivationPath::from_str("m/44'/5757'/0'/0/0")?;
    let child_key = master_key.derive_priv(&secp, &path)?;
    println!("{}",hex::encode(child_key.private_key.secret_bytes()));
    Ok(hex::encode(child_key.private_key.secret_bytes()))
}   

pub fn derive_btc_address_from_private_key(private_key: &str,) -> Result<String, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_str(private_key)?;

    // Derive the compressed public key
    let public_key = secret_key.public_key(&secp);

    // Create Bitcoin testnet address (since we're using testnet in the keychain.json)
    let pubkey = bitcoin::PublicKey::new(public_key);
    let address = bitcoin::Address::p2pkh(&pubkey, Network::Testnet);

    Ok(address.to_string())
}

pub fn get_stx_adress(mnemonic: &str) -> Result<AddressData, Box<dyn std::error::Error>> {
    let private_key = derive_private_key_from_mnemonic(mnemonic)?;

    let btc_address = derive_btc_address_from_private_key(&private_key)?;
    convert_btc_stx_adress(&btc_address)?;
    let file_content = match std::fs::read_to_string("stx_address.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", error_m!("Failed to read stx_address.json"));
            return Err(e.into());
        }
    };
    let address_data: AddressData = serde_json::from_str(&file_content)?;

    Ok(address_data)
}
