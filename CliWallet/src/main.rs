use cli_wallet::console::run;
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => println!("{}", e.to_string()),
    }

    create_logs_table();
}

fn create_logs_table() {
    let conn = match Connection::open("logs.db"){
        Ok(conn) => conn,
        Err(e) => {
            println!("Error opening database: {}", e);
            return;
        }
    };

    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            adress TEXT PRIMARY KEY,
            auth_type TEXT
        )",
        [],
    ).unwrap();
}
