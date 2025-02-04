mod args;
mod console;
mod fetch;
mod error;
mod file_io;
mod weather;

#[tokio::main]
async fn main() {
    console::flow().await;
}


