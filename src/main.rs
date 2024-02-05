use ntex::web::{self, middleware, App, HttpResponse};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    web::HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().finish() }))
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 6476))?
    .run()
    .await
}
