use bitcoin_handshake_cli::{Config, run};
use rusqlite::{Connection, Result};
use colored::*;

macro_rules! warn_m {
    ($name:expr) => {
        format!("WARN: {}!", $name).yellow()
    };
}       

macro_rules! error_m {
    ($name:expr) => {
        format!("ERR: {}", $name).red()
    };
}


#[tokio::main]
async fn main(){

    
    let cli_arguments = std::env::args();

    let config = match Config::from(cli_arguments) {
        Ok(config) => config,
        Err(e) => {
            println!("{}",warn_m!(format!("Problem parsing arguments: {:?}", e.to_string())));
            return
        }
    };

    if let Err(e) = run(&config).await {
        println!("{}",error_m!(format!("Error while running the {:?}: {:?}", config.command, e.to_string())));
    }

    use chrono::prelude::*;

    let _local: DateTime<Local> = Local::now();
    println!("{_local:?}");

    create_logs_table();
    get_succesfull().unwrap();
    
}

 fn create_logs_table() {
    let conn = Connection::open("logs.db").unwrap();
    
    // Create table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dns TEXT,
            date TEXT,
            ip TEXT,
            version INTEGER,
            success BOOLEAN
        )",
        [],
    ).unwrap();
}

#[derive(Debug)]
pub struct SuccessfullHandshake {
    pub ip: String,
    pub dns: String,
    pub version: u32,
    pub date: String,

}

fn get_succesfull() -> Result<()> {
    let conn = Connection::open("logs.db")?;
    let mut stmt = conn.prepare("SELECT dns, date, ip, version FROM logs WHERE success = 1")?;
    let logs_iter = stmt.query_map([], |row| {
        Ok(SuccessfullHandshake{
            dns: row.get(0)?,
            date: row.get(1)?,
            ip: row.get(2)?,
            version: row.get(3)?,
        })
    })?;


    Ok(())
}

