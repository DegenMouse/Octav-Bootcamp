use rocket::{get, routes, Build};
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::quotes::Quote;
use rocket::serde::json::Json;
use rocket::http::Method;

fn cors_options() -> CorsOptions {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get, Method::Post].into_iter().map(From::from).collect())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/quotes/<number>")]
async fn quotes(number: u32) -> Result<Json<Vec<Quote>>, Json<String>> {
    let mut quotes = Vec::new();
    for _ in 0..number {
        match Quote::fetch().await {
            Ok(quote) => quotes.push(quote),
            Err(e) => return Err(Json(format!("Failed to fetch quote: {}", e))),
        }
    }
    Ok(Json(quotes))
}

pub async fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, quotes])
        .attach(cors_options().to_cors().unwrap())
}