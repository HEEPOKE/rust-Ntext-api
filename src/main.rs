mod configs;
mod schemas;
mod models;
use ntex::web::{self, middleware, App, HttpResponse};

use configs::configs::CONFIG;
use configs::database::db_connection;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let port: u16 = CONFIG
        .port
        .clone()
        .parse()
        .expect("Failed to parse port number");
    let host = CONFIG.host.clone();
    let database_url = CONFIG.database_url.clone();

    web::HttpServer::new(move || {
        let connection = db_connection(&database_url);
        App::new()
            .state(connection)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { HttpResponse::Ok().finish() }))
    })
    .bind((host, port))?
    .run()
    .await
}
