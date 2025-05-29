// use fetch::get_weather;

mod args;
mod console;
mod fetch;
mod error;
mod file_io;
mod weather;

#[tokio::main]
async fn main() {
    console::flow().await;
    let data = fetch::get_weather("London").await;
    println!("{:?}", data);
}


