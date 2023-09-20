use actix_web::{App, HttpServer};
use log::info;

mod app;

#[derive(Debug)]
pub struct CfgServer {
    port: u16,
    host: String,
}

impl CfgServer {
    pub fn new() -> CfgServer {
        CfgServer {
            ..Default::default()
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        info!("Starting server on {}:{}", self.host, self.port);

        HttpServer::new(|| App::new().service(app::health_check).service(app::config))
            .bind((self.host.as_str(), self.port))?
            .run()
            .await
    }
}

impl Default for CfgServer {
    fn default() -> Self {
        CfgServer {
            port: 8080,
            host: "127.0.0.1".to_string(),
        }
    }
}
