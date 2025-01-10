use crate::config::{
    derive_btc_address_from_private_key, derive_private_key_from_mnemonic, get_stx_adress,
};
use crate::consts::{AuthData, Keychain};
use crate::{error_m, info_m};
use std::process::Command;
use std::{fs::File, io, io::Write};

fn create_keychain() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("sh")
        .arg("-c")
        .arg("stx make_keychain | jq > keychain.json") // Run the shell command with output redirection
        .status()?; // Get the status of the command

    // Check if the command ran successfully
    if !status.success() {
        eprintln!("{}", error_m!("Failed to execute `stx make_keychain`"));
        return Err("Failed to execute command".into());
    }

    println!("\n{}", info_m!("New keychain created successfully and saved to keychain.json, please backup your mnemonic phrase"));
    Ok(())
}

fn get_intput(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().to_string()
}

//just testnet for now
pub fn login_with_mnemonic() -> Result<AuthData, Box<dyn std::error::Error>> {
    let mnemonic = get_intput("Paste your mnemonic phrase: ");
    let address_data = get_stx_adress(&mnemonic)?;
    let private_key = derive_private_key_from_mnemonic(&mnemonic)?;
    let btc_address = derive_btc_address_from_private_key(&private_key)?;
    Ok(AuthData {
        mnemonic,
        private_key,
        stx_address: address_data.testnet.stacks.clone(),
        btc_address,
    })
}

pub fn create_new_wallet() -> Result<AuthData, Box<dyn std::error::Error>> {
    create_keychain()?;
    let file = File::open("keychain.json")?;
    let keychain: Keychain = serde_json::from_reader(file)?;
    let auth_data = AuthData {
        mnemonic: keychain.mnemonic,
        private_key: keychain.key_info.private_key,
        stx_address: keychain.key_info.address.clone(),
        btc_address: keychain.key_info.btc_address,
    };
    Ok(auth_data)
}
