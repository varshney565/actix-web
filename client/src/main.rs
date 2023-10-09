pub mod controller;
pub mod utils;
use actix_web::{HttpServer, web, App};
use controller::reply::reply;
use controller::proposal::index;
use utils::remote_address::add_node;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
    if environment == "dev" {
        dotenv::from_filename("client.env").ok();
    }
    let port = env::var("PORT").expect("Failed to load the Port !!");
    add_node("192.168.178.32:8081");
    add_node("192.168.178.32:8080");
    add_node("192.168.178.32:8082");
    add_node("192.168.178.32:8083");
    add_node("192.168.178.32:8084");
    HttpServer::new(||
        App::new()
            .route("/proposal",web::post().to(index))
            .route("/reply",web::post().to(reply))
    )                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
