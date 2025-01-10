use bip39::{Error as Bip39Error, Mnemonic};
use bitcoin::bip32::{DerivationPath, Xpriv, Error as Bip32Error};
use bitcoin::secp256k1::{Secp256k1, SecretKey, Error as Secp256k1Error};
use bitcoin::{Network, PublicKey};
use std::str::FromStr;
use thiserror::Error;
use stacks_core::address::{StacksAddress, AddressVersion};

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("BIP39 error: {0}")]
    Bip39Error(#[from] Bip39Error),
    #[error("BIP32 error: {0}")]
    Bip32Error(#[from] Bip32Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(#[from] Secp256k1Error),
    #[error("Failed to generate account: {0}")]
    AccountGenerationError(String),
    #[error("Failed to parse address: {0}")]
    AddressError(String),
    #[error("Invalid coin type: {0}")]
    InvalidCoinType(String),
    #[error("Invalid hex string: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("Invalid public key: {0}")]
    StacksPublicKeyError(#[from] stacks_core::crypto::secp256k1::Error),
}

#[derive(Debug)]
pub struct Account {
    pub mnemonic: String,
    pub private_key: String,
    pub public_key: String,
    pub mainnet_address: String,
    pub testnet_address: String,
    pub mainnet_stx_address: String,
    pub testnet_stx_address: String,
}

pub fn generate_seed() -> Result<(String, [u8; 64]), WalletError> {
    let mnemonic = Mnemonic::generate(24)?;
    let passphrase = "";
    Ok((mnemonic.to_string(), mnemonic.to_seed(passphrase)))
}

pub fn generate_seed_from_mnemonic(mnemonic: &str) -> Result<[u8; 64], WalletError> {
    let mnemonic = Mnemonic::from_str(mnemonic)?;
    let passphrase = "";
    Ok(mnemonic.to_seed(passphrase))
}

pub fn generate_private_key(seed: [u8; 64], index: u32, coin_type: &str) -> Result<String, WalletError> {
    let secp = Secp256k1::new();
    let master_key = Xpriv::new_master(Network::Bitcoin, &seed)?;
    
    let path = if coin_type == "btc" {
        DerivationPath::from_str(format!("m/44'/0'/0'/0/{}", index).as_str())?
    } else if coin_type == "stx" {
        DerivationPath::from_str(format!("m/44'/5757'/0'/0/{}", index).as_str())?
    } else {
        return Err(WalletError::InvalidCoinType("Invalid coin type".to_string()));
    };
    
    let child_key = master_key.derive_priv(&secp, &path)?;
    Ok(hex::encode(child_key.private_key.secret_bytes()))
}

pub fn generate_public_key(private_key: &str) -> Result<String, WalletError> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_str(private_key)?;
    let public_key = secret_key.public_key(&secp);
    // Return compressed public key format (33 bytes) instead of uncompressed (65 bytes)
    Ok(hex::encode(public_key.serialize()))
}
//genereaza ambele adrese
pub fn generate_btc_address(public_key: &str) -> Result<(String, String), WalletError> {
    let pubkey = PublicKey::from_str(public_key)
        .map_err(|e| WalletError::AddressError(e.to_string()))?;
    let mainnet_address = bitcoin::Address::p2pkh(&pubkey, Network::Bitcoin);
    let testnet_address = bitcoin::Address::p2pkh(&pubkey, Network::Testnet);
    Ok((mainnet_address.to_string(), testnet_address.to_string()))
}

pub fn generate_stx_address(public_key: &str) -> Result<(String,String), WalletError> {
    // Example public key in hex format
    let public_key_bytes = hex::decode(public_key)?;
    use stacks_core::crypto::secp256k1::PublicKey as StacksPublicKey;
    let public_key = StacksPublicKey::from_slice(&public_key_bytes)?;

    // Create a Stacks address using the public key and version
    let mainnet_address = StacksAddress::from_public_key(AddressVersion::MainnetSingleSig, &public_key);
    let testnet_address = StacksAddress::from_public_key(AddressVersion::TestnetSingleSig, &public_key);

    Ok((mainnet_address.to_string(), testnet_address.to_string()))
}

pub fn generate_account() -> Result<Account, WalletError> {
    let seed = generate_seed()?;
    let private_key = generate_private_key(seed.1, 0, "btc")?;
    let public_key = generate_public_key(&private_key)?;
    let (mainnet_address, testnet_address) = generate_btc_address(&public_key)?;
    let (mainnet_stx_address, testnet_stx_address) = generate_stx_address(&public_key)?;
    Ok(Account {
        mnemonic: seed.0,
        private_key,
        public_key,
        mainnet_address,
        testnet_address,
        mainnet_stx_address,
        testnet_stx_address,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_seed() {
        let seed = generate_seed().expect("Failed to generate seed");
    }

    #[test]
    fn test_generate_private_key() {
        let seed = generate_seed().expect("Failed to generate seed");
        let private_key = generate_private_key(seed.1, 0, "btc").expect("Failed to generate private key");
    }

    #[test]
    fn test_generate_public_key() {
        let seed = generate_seed().expect("Failed to generate seed");
        let private_key = generate_private_key(seed.1, 0, "btc").expect("Failed to generate private key");
        let public_key = generate_public_key(&private_key).expect("Failed to generate public key");
    }

    #[test]
    fn test_generate_seed_from_mnemonic() {
        let mnemonic = "oyster mirror pole knee shock easy panda toast category denial dutch guard left mail ticket clerk twelve neutral limb coast squirrel attack copy summer";
        let seed = generate_seed_from_mnemonic(mnemonic).expect("Failed to generate seed");
        let private_key = generate_private_key(seed, 0, "stx").expect("Failed to generate private key");
        let public_key = generate_public_key(&private_key).expect("Failed to generate public key");
        let stx_address = generate_stx_address(&public_key).expect("Failed to generate STX address");
        assert_eq!(stx_address.1, "ST2F66ASMYZ9M8EEVD4S76RCF9X15WZD2EPE5RR75");
    }

    #[test]
    fn test_generate_stx_address() {
        let seed = generate_seed().expect("Failed to generate seed");
        let private_key = generate_private_key(seed.1, 0, "stx").expect("Failed to generate private key");
        let public_key = generate_public_key(&private_key).expect("Failed to generate public key");
        let address = generate_btc_address(&public_key).expect("Failed to generate BTC address");
    }

    #[test]
    fn test_generate_account() {
        let account = generate_account().expect("Failed to generate account");
        println!("Account:");
        println!("  Mnemonic: {}", account.mnemonic);
        println!("  Private Key: {}", account.private_key);
        println!("  Public Key: {}", account.public_key);
        println!("  Mainnet Address: {}", account.mainnet_address);
        println!("  Testnet Address: {}", account.testnet_address);
        println!("  Mainnet STX Address: {}", account.mainnet_stx_address);
        println!("  Testnet STX Address: {}", account.testnet_stx_address);
    }

}
