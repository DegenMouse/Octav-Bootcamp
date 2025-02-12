use std::io::{BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::sync::Arc;
use tokio::time::timeout;
use std::time::Duration;
use bitcoin::consensus::Decodable;
use bitcoin::network::message::{NetworkMessage, RawNetworkMessage};
use crate::config::CliNetwork;
use crate::network::{get_version_message, get_verack_message};
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use colored::*;


#[derive(Clone)]
pub struct HandshakeResult {
    pub ip: String,
    pub dns: String,
    pub version: u32,
    pub date: String,
    pub success: bool,
}

macro_rules! error_m {
    ($name:expr) => {
        format!("ERR: {}", $name).red()
    };
}
macro_rules! info_m {
    ($name:expr) => {
        format!("INFO: {}", $name).green()
    };
}

#[derive(thiserror::Error, Debug)]
pub enum HandshakeError {
    #[error("Failed to clone stream: {0}")]
    StreamCloneError(String),
    #[error("Failed to retrieve address: {0}")]
    AddressRetrieveError(String),
    #[error("Failed to send version message: {0}")]
    VersionSendError(String),
    #[error("Failed to send verack message: {0}")]
    VerackSendError(String),
    #[error("Failed to decode version message from remote: {0}")]
    VersionDecodeError(String),
    #[error("Failed to decode verack message from remote: {0}")]
    VerackDecodeError(String),
    #[error("Remote hasn't sent a verack message back!")]
    NoVerackResponse,
    #[error("Remote hasn't sent a version message back!")]
    NoVersionResponse,
    #[error("Failed to connect to node: {0}")]
    NodeConnectionError(String),
    #[error("Error running handshake: {0}")]
    HandshakeRuntimeError(String),
    #[error("Failed to join handshake thread: {0}")]
    JoinHandleError(String),
    #[error("Handshake timed out.")]
    HandshakeTimeout,
}

pub async fn run_handshake(ip_list: Vec<SocketAddr>, print_in_file: bool, network: Arc<CliNetwork>) {
    let mut output_writer: Box<dyn Write> = if print_in_file {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("output.log")
        {
            Ok(file) => {
                if let Err(e) = file.set_len(0) {
                    println!("{}",error_m!(format!("Failed to clear file contents: {}", e)));
                }
                Box::new(file)
            },
            Err(e) => {
                println!("{}",error_m!(format!("Failed to open output.log: {}", e)));
                return;
            }
        }
    } else {
        Box::new(std::io::stdout())
    };
    for ip in ip_list {
        let handshake_result = Arc::new(Mutex::new(HandshakeResult {
            ip: ip.to_string(),
            dns: String::from(""),
            version: 0,
            date: String::from(""),
            success: true,
        }));
        
        log_message(&mut output_writer, info_m!(format!("Performing handshake for {:?}:", &ip)));
        let network_arc = Arc::clone(&network);
        let handshake_result_arc = Arc::clone(&handshake_result);

        let handshake = tokio::spawn(async move {
            do_handshake(ip, print_in_file.clone(), network_arc, handshake_result_arc).await
        });

        let timeout = timeout(Duration::from_secs(4), handshake).await;
        let timeout_result = match timeout {
            Ok(result) => {
                match result {
                    Ok(handshake) => {
                        match handshake {
                            Ok(()) => Ok(()),
                            Err(e) => {
                                Err(HandshakeError::HandshakeRuntimeError(e.to_string()))
                            }
                        }
                    }
                    Err(e) => {
                        if let Ok(mut hr) = handshake_result.lock() {
                            hr.success = false;
                            let _ = add_data_to_db(hr.clone()); 
                        }
                        Err(HandshakeError::JoinHandleError(e.to_string()))
                    }
                }
            }
            Err(_) => Err(HandshakeError::HandshakeTimeout),
        };

        if let Err(e) = timeout_result {
            log_message(&mut output_writer,error_m!(e.to_string().add("\n")));
        }
    }
}

fn log_message(output_writer: &mut Box<dyn Write>, message: impl std::fmt::Display) {
    if let Err(err) = writeln!(output_writer, "{}", message) {
        println!("{}",error_m!(format!("Failed to log message: {}", err)));
    }
}

async fn do_handshake(ip: SocketAddr, print_in_file: bool, network: Arc<CliNetwork>, handshake_result: Arc<Mutex<HandshakeResult>>) -> Result<(), HandshakeError> {
    
    let mut log_writer: Box<dyn Write> = if print_in_file {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("output.log")
        {
            Ok(file) => Box::new(file),
            Err(e) => {
                println!("{}",error_m!(format!("Failed to open output.log: {}", e)));
                Box::new(std::io::stdout())
            }
        }
    } else {
        Box::new(std::io::stdout())
    };

    return match TcpStream::connect(ip) {
        
        Ok(mut stream) => {
            let read = match stream.try_clone() {
                Ok(stream) => stream,
                Err(e) => {
                    return Err(HandshakeError::StreamCloneError(e.to_string()))
                }
            };

            let mut buf_reader = BufReader::new(read);

            let local_address = match stream.local_addr() {
                Ok(address) => address,
                Err(e) => {
                    return Err(HandshakeError::AddressRetrieveError(e.to_string()));
                }
            };

            let remote_address = match stream.peer_addr() {
                Ok(address) => address,
                Err(e) => {
                    return Err(HandshakeError::AddressRetrieveError(e.to_string()));
                }
            };

            let version_message_local = get_version_message(local_address, remote_address, &network);

            if let Err(e) = stream.write_all(version_message_local.as_slice()) {
                return Err(HandshakeError::VersionSendError(e.to_string()));
            };

            log_message(&mut log_writer, info_m!(format!("Sent version message to {:?}!", remote_address)));

            match RawNetworkMessage::consensus_decode(&mut buf_reader) {
                Ok(decoded_message) => {
                    match decoded_message.payload {
                        NetworkMessage::Version(message) => {
                            log_message(&mut log_writer, info_m!(format!("Got version message back: {}", message.version)));
                            let verack_message_local = get_verack_message(&network);

                            if let Err(e) = stream.write_all(verack_message_local.as_slice()) {
                                return Err(HandshakeError::VerackSendError(e.to_string()));
                            };

                            log_message(&mut log_writer, info_m!(format!("Sent verack message to {:?}!", remote_address)));
                            
                            let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
                            let formatted_date = local.format("%H:%M:%S").to_string();
                            if let Ok(mut hr) = handshake_result.lock() {
                                hr.date = formatted_date;
                                hr.version = message.version;
                            }

                            match RawNetworkMessage::consensus_decode(&mut buf_reader) {
                                Ok(decoded_message) => {
                                    match decoded_message.payload {
                                        NetworkMessage::Verack => {
                                            log_message(&mut log_writer, info_m!("Got verack message back.\n"));
                                            if let Ok(mut hr) = handshake_result.lock() {
                                                hr.success = true;
                                                let _ = add_data_to_db(hr.clone());
                                            }
                                            Ok(())
                                        }
                                        _ => {
                                            Err(HandshakeError::NoVerackResponse)
                                        }
                                    }
                                }
                                Err(e) => {
                                    Err(HandshakeError::VerackDecodeError(e.to_string()))
                                }
                            }
                        }
                        _ => {
                            if let Ok(mut hr) = handshake_result.lock() {
                                hr.success = false;
                                let _ = add_data_to_db(hr.clone());
                            }
                            Err(HandshakeError::NoVersionResponse)
                        }
                    }
                }
                Err(e) => {
                    if let Ok(mut hr) = handshake_result.lock() {
                        hr.success = false;
                        hr.dns = String::from("");
                        let _ = add_data_to_db(hr.clone());
                    }
                    Err(HandshakeError::VersionDecodeError(e.to_string()))
                }
            }
            
            
        }
        Err(e) => {
            
            if let Ok(mut hr) = handshake_result.lock() {
                hr.success = false;
                let _ = add_data_to_db(hr.clone());
            }
            Err(HandshakeError::NodeConnectionError(e.to_string()))
        }   
    }
    
}



fn add_data_to_db(handshake_result: HandshakeResult) -> Result<()> {
    let conn = Connection::open("logs.db")?;
    conn.execute(
        "INSERT INTO logs (dns, date, ip, version, success) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            handshake_result.dns,
            handshake_result.date,
            handshake_result.ip,
            handshake_result.version,
            handshake_result.success
        ],
    )?;

    Ok(())
}
