use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Config<'a> {
    server: ServerConfig<'a>,
    database: DatabaseConfig<'a>,
}

#[derive(Debug, Serialize)]
struct ServerConfig<'a> {
    host: &'a str,
    port: u16,
}

#[derive(Debug, Serialize)]
struct DatabaseConfig<'a> {
    url: &'a str,
    username: &'a str,
    password: &'a str,
}

static PUBLIC_CONFIG: Config = Config {
    server: ServerConfig {
        host: "127.0.0.1",
        port: 8080,
    },
    database: DatabaseConfig {
        url: "postgres://localhost:5432",
        username: "postgres",
        password: "postgres",
    },
};

#[get("/healthz")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/config")]
pub async fn config() -> impl Responder {
    web::Json(&PUBLIC_CONFIG)
}
