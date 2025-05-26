mod web;
mod quotes;

#[rocket::main]
async fn main() {
    println!("Launching Rocket server...");
    let _ = web::rocket().await.launch().await;
}