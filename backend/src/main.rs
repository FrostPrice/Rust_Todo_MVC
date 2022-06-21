#![allow(unused)] // This will silence unused warnings while exploring (This will be comment out)

use std::{env, sync::Arc};

use model::init_db;
use web::start_web;

mod model; // Data Layer
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    // Compute the web_folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = DEFAULT_WEB_PORT;

    // Get the Database
    // TODO - Loop until you get a valid DB
    let db = init_db().await.expect("Cannot init DB");
    let db = Arc::new(db);

    // Start the Server
    match start_web(&web_folder, web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("ERROR - Web Server failed to start. Cause {:?}", ex),
    }
}
