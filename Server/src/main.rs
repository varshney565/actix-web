use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/yes")]
async fn index1() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "YES"
}

#[get("/no")]
async fn index2() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "NO"
}

#[get("/yes1")]
async fn index3() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "YES"
}

#[get("/no1")]
async fn index4() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "NO"
}

#[get("/yes3")]
async fn index5() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "YES"
}

#[get("/no3")]
async fn index6() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "NO"
}

#[get("/yes2")]
async fn index7() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "YES"
}

#[get("/no2")]
async fn index8() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
    "NO"
}

#[get("/final1")]
async fn index9() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    "YES"
}

#[get("/final2")]
async fn index10() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    "NO"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index1)
            .service(index2)
            .service(index3)
            .service(index4)
            .service(index5)
            .service(index6)
            .service(index7)
            .service(index8)
            .service(index9)
            .service(index10)
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}