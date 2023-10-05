pub mod controller;

pub mod utils;

use actix_web::{web, App, HttpServer};

use controller::accept_proposal_secondry::secondry_index;

use controller::accept_proposal::index;

use controller::receive_signal::receive_signal;

use controller::health::health_check;

use controller::vote::vote;

use utils::ips::add_node;

use std::env;

use dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
    if environment == "dev" {
        dotenv::from_filename("node4.env").ok();
    }
    let port = env::var("PORT").expect("Failed to load the Port !!");
    add_node("172.16.14.113:8081".to_string());
    add_node("172.16.14.113:8080".to_string());
    add_node("172.16.14.113:8082".to_string());
    add_node("172.16.14.113:8083".to_string());
    add_node("172.16.14.113:8084".to_string());
    HttpServer::new(|| 
        App::new()
                .route("/proposal", web::post().to(index))
                .route("/running",web::head().to(health_check))
                .route("/node", web::post().to(secondry_index))
                .route("/vote", web::post().to(vote))
                .route("/signal", web::post().to(receive_signal))
    )
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}