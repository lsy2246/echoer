mod utils;
use rocket::{get, routes};
use surrealdb;
use utils::error::CustomResult;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/theme")]
fn get_theme() -> &'static str {
    "light"
}

struct Appstate{
    
}

#[rocket::main]
async fn main() -> CustomResult<()> {
    let rocket_build = rocket::build();

    let rocket = rocket_build
        .mount("/", routes![index, get_theme])
        .ignite().await?;
    rocket.launch().await?;
    std::process::exit(0);
} 