use actix_web::{get, App, HttpServer, Responder};

#[get("/yes")]
async fn index1() -> impl Responder {
    let id = std::thread::current().id();
    println!("{:?}",id);
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    "YES"
}

#[get("/no")]
async fn index2() -> impl Responder {
    let id = std::thread::current().id();
    println!("{:?}",id);
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    "NO"
}

#[get("/ok")]
async fn index3() -> impl Responder {
    let id = std::thread::current().id();
    println!("{:?}",id);
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    "YES"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let id = std::thread::current().id();
    println!("{:?}",id);
    HttpServer::new(|| {
        App::new()
            .service(index1)
            .service(index2)
            .service(index3)
    })
    .workers(1)
    .worker_max_blocking_threads(1)
    .client_request_timeout(std::time::Duration::from_secs(40000))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}