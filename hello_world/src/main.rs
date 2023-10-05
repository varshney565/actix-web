// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/hey", web::get().to(manual_hello))
//             .service(hello)
//             .service(echo)
            
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{web, App, HttpServer};

// This struct represents state
struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

async fn index1(data : web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Bye {app_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/hello")
                    .app_data(web::Data::new(AppState {
                        app_name: String::from("Actix Web"),
                    }))
                    .route("world",web::get().to(index))
                    .route("working",web::get().to(index1))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}