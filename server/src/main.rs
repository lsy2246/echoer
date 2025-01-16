mod db;
mod utils;

use db::Database;
use rocket::{get, http::Method, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use utils::error::CustomResult;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct AppState {
    db: Database,
}

fn cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        expose_headers: Default::default(),
        max_age: None,
        send_wildcard: false,
        fairing_route_base: "/".to_string(),
        fairing_route_rank: 0,
    }
    .to_cors()
    .expect("CORS配置错误")
}

#[rocket::main]
async fn main() -> CustomResult<()> {
    let rocket_build = rocket::build();

    let db = Database::link().await?;

    let rocket = rocket_build
        .mount("/", routes![index])
        .manage(AppState { db })
        .attach(cors())
        .ignite()
        .await?;
    rocket.launch().await?;
    std::process::exit(0);
}
