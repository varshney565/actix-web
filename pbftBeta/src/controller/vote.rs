
use actix_web::HttpResponse;

pub async fn vote() -> HttpResponse {
    HttpResponse::Ok().json("voting is successful")
}