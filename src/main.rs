mod configs;
mod schemas;
mod models;

use ntex::web::{self, middleware, App, HttpResponse};
use env_logger::Env;

use configs::configs::CONFIG;
use configs::database::db_connection;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = CONFIG.port.parse().unwrap_or_else(|_| {
        eprintln!("Invalid port number specified. Using default port 6476.");
        6476
    });
    let database_url = &CONFIG.database_url;

    web::HttpServer::new(move || {
        let connection = db_connection(database_url);
        App::new()
            .state(connection.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(|| async { HttpResponse::Ok().finish() }))
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
