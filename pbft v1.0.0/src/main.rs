pub mod controller;
pub mod utils;
use actix_web::{web, App, HttpServer};

use crate::controller::vote::vote;
use utils::ips::REMOTE_ADDRESS;
use controller::accept_proposal::index;

fn add_node(node : &'static str ) {
    unsafe {
        REMOTE_ADDRESS.push(node);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    add_node("172.16.14.113:8081");
    add_node("172.16.14.113:8080");
    add_node("172.16.14.113:8082");
    add_node("172.16.14.113:8083");
    HttpServer::new(|| 
        App::new()
                .route("/proposal", web::post().to(index))
                .route("/node", web::post().to(vote))
    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}