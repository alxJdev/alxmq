use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("Server Online")
}