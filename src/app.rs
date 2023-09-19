use actix_web::{get, HttpResponse, Responder};

#[get("/healthz")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
