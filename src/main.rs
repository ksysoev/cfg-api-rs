#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = cfg_api::CfgServer::new();
    server.run().await
}
