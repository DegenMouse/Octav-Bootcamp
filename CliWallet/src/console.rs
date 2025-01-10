use crate::auth::{create_new_wallet, login_with_mnemonic};
use crate::calls::{get_account_info, request_stx_tokens};
use crate::consts::AuthData;
use crate::{error_m, info_m, warn_m};
use nu_ansi_term::Color;
use reqwest::Error as ReqwestError;
use std::io::{self, Write};
use rusqlite::{params, Error as SqliteError, Connection};

#[derive(thiserror::Error, Debug)]
pub enum TestErrors {
    #[error("Mnemonic is invalid")]
    InvalidMnemonic(Box<dyn std::error::Error>),
    #[error("Failed to get STX balance")]
    StxBalanceFailed(ReqwestError),
    #[error("Failed to create new wallet")]
    CreateNewWalletFailed(Box<dyn std::error::Error>),
    #[error("Invalid choice")]
    InvalidChoice,
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqliteError),
}

pub fn print_logged_in_menu() {
    println!("{} Get STX balance (testnet)", Color::Rgb(255, 140, 0).paint("[1]"));
    println!("{} Get STX balance (mainnet)", Color::Rgb(255, 140, 0).paint("[2]"));
    println!("{} Add faucet STX to testnet wallet", Color::Rgb(255, 140, 0).paint("[3]"));
    println!("{} Logout", Color::Rgb(255, 140, 0).paint("[4]"));
    println!("[0] Exit");
}

pub fn print_logged_out_menu() {
    println!("{}", Color::Rgb(255, 140, 0).paint("\nWelcome to the Stacks Wallet"));
    println!("{} Add wallet with secret key", Color::Rgb(255, 140, 0).paint("[1]"));
    println!("{} Create new wallet", Color::Rgb(255, 140, 0).paint("[2]"));
    println!("[0] Exit");
}

fn get_intput(text: &str) -> u32 {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    let result = match inp.trim().parse::<u32>() {
        Ok(input) => input,
        Err(_) => {
            println!("{}", warn_m!("Invalid choice"));
            get_intput(text)
        }
    };
    result
}

pub async fn run() -> Result<(), TestErrors> {
    let mut logged_in: bool = false;
    let mut auth_data: AuthData = AuthData {
        mnemonic: String::new(),
        private_key: String::new(),
        stx_address: String::new(),
        btc_address: String::new(),
    };
    let mut login_type: String = String::new();
    loop {
        if logged_in {
            add_log(&auth_data.stx_address,&login_type).unwrap();
            print_logged_in_menu();
            let input: u32 = get_intput("Enter your choice: ");
            match input {
                1 => {
                    let balance = match get_account_info(&auth_data.stx_address, "testnet").await {
                        Ok(balance) => balance,
                        Err(e) => {
                            println!("{}", error_m!(TestErrors::StxBalanceFailed(e)));
                            continue;
                        }
                    };
                    println!("\n{}", info_m!(format!("STX balance: {}", balance)));
                }
                2 => {
                    let balance = match get_account_info(&auth_data.stx_address, "mainnet").await {
                        Ok(balance) => balance,
                        Err(e) => {
                            return Err(TestErrors::StxBalanceFailed(e));
                        }
                    };
                    println!("\n{}", info_m!(format!("STX balance: {}", balance)));
                }
                3 => {
                    match request_stx_tokens(&auth_data.stx_address).await {
                        Ok(_) => println!("{}", info_m!("STX tokens requested successfully")),
                        Err(e) => println!("{}", error_m!(e)),
                    }
                }
                4 => {
                    logged_in = false;
                    println!("{}", info_m!("Successfully logged out"));
                }
                0 => {
                    break;
                }
                _ => {
                    return Err(TestErrors::InvalidChoice);
                }
            }
        } else {
            print_logged_out_menu();
            let input: u32 = get_intput("Enter your choice: ");
            match input {
                1 => {
                    auth_data = match login_with_mnemonic() {
                        Ok(auth_data) => auth_data,
                        Err(e) => {
                            println!("{}", error_m!(TestErrors::InvalidMnemonic(e)));
                            continue;
                        }
                    };
                    logged_in = true;
                    login_type = String::from("Login by private key");
                    println!("\n{}", info_m!("Successfully logged in"));
                }
                2 => {
                    auth_data = match create_new_wallet() {
                        Ok(auth_data) => auth_data,
                        Err(e) => {
                            return Err(TestErrors::CreateNewWalletFailed(e));
                        }
                    };
                    logged_in = true;
                    login_type = String::from("Login by wallet creation");
                    println!("{}", info_m!("Successfully logged in"));
                }
                0 => {
                    break;
                }
                _ => {
                    return Err(TestErrors::InvalidChoice);
                }
            }
        }
    }
    Ok(())
}

fn add_log(address: &str, auth_type: &str) -> Result<(), SqliteError> {
    let conn = Connection::open("logs.db")?;
    
    // First check if address already exists
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM logs WHERE adress = ?")?;
    let count: i64 = stmt.query_row(params![address], |row| row.get(0))?;
    
    // Only insert if address doesn't exist
    if count == 0 {
        conn.execute(
            "INSERT INTO logs (adress, auth_type) VALUES (?1, ?2)",
            params![address, auth_type],
        )?;
    }
    
    Ok(())
}