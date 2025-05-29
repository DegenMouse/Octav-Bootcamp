#[macro_use] extern crate rocket;

mod routes;
mod services;
mod weather;
mod error;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

// import our routes
use routes::current_forecast::{get_current_weather, basic};
use routes::weekly_forecast::{get_weekly_weather};

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![get_current_weather, get_weekly_weather])
        .attach(cors.to_cors().unwrap())
}