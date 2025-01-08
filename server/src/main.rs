mod db;
mod utils;

use db::Database;
use rocket::{get, routes, State};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use utils::error::CustomResult;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/theme")]
fn get_theme() -> &'static str {
    "light"
}

pub struct AppState {
    db: Database,
}

#[rocket::main]
async fn main() -> CustomResult<()> {
    let rocket_build = rocket::build();

    let db = Database::link().await?;

    let rocket = rocket_build
        .mount("/", routes![index, get_theme])
        .manage(AppState { db })
        .ignite()
        .await?;
    rocket.launch().await?;
    std::process::exit(0);
}
