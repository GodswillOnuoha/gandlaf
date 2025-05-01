#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// FIXME: The code above should only be used in development.

mod config;

use std::net::TcpListener;

use config::database;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_connection_pool = database::get_db_connection_pool().await;

    Ok(())
}
