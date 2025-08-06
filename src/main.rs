mod auth;
mod handler;
mod middleware;

use actix_web::{dev::{fn_service, ServiceRequest, ServiceResponse}, middleware::{from_fn, Logger, NormalizePath, TrailingSlash}, web, App, HttpServer};
use actix_files::Files;

use log::LevelFilter;

const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100 MB

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
    .filter_level(LevelFilter::Info)
    .filter_module("actix_server", LevelFilter::Off)
    .init();

    log::info!("Starting...");
    log::info!("Loading env vars...");
    dotenv::dotenv().ok();
    log::info!("Env vars loaded.");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    log::info!("Listening on PORT {:?}", port);
    let secret  = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        log::error!("JWT_SECRET environment variable is not set");
        std::process::exit(1);
    });

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "static").index_file("404.html").default_handler(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let file = actix_files::NamedFile::open_async("./static/404.html").await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            })))
            .service(
                web::scope("")
                    .service(handler::get_file)
                    .service(handler::get_dir)
                    .wrap(from_fn(middleware::auth_middleware))
            )
            .app_data(web::Data::new(secret.clone()))
            .app_data(web::PayloadConfig::new(MAX_FILE_SIZE))
            .wrap(Logger::new("%a %r %s %Dms"))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
        })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
