use actix_files::{Files, NamedFile};
use actix_web::{
    App, HttpServer,
    dev::{ServiceRequest, ServiceResponse, fn_service},
    middleware::{Logger, NormalizePath, TrailingSlash, from_fn},
    web,
};
use env_logger::Env;
use log::info;
use rodan_fileserver::{config, handler, types, utils::middlewares::auth_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting file server...");
    dotenv::dotenv().ok();

    let cfg_file = std::env::var("CONFIG_FILE").expect("Missing CONFIG_FILE environment variable");
    let cfg = config::Config::from_file(&cfg_file)
        .await
        .expect("Failed to load configuration");
    let jwt_secret = cfg.server.jwt_secret.clone();
    let folder_path = cfg.server.folder_path.clone();
    let max_payload_bytes = cfg.server.max_payload_bytes();

    let addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    info!("Server listening on {}", addr);
    info!("Serving files from: {}", folder_path);
    let shared_secret = web::Data::new(types::JwtSecret(jwt_secret));
    let shared_folder_path = web::Data::new(types::FolderPath(folder_path));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(shared_secret.clone())
            .app_data(shared_folder_path.clone())
            .app_data(web::PayloadConfig::new(max_payload_bytes))
            .wrap(Logger::new("%a %r %s %Dms"))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(
                Files::new("/static", "static")
                    .index_file("404.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./static/404.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
            .service(
                web::scope("")
                    .service(handler::get_file)
                    .service(handler::get_dir)
                    .wrap(from_fn(auth_middleware)),
            )
    })
    .bind(&addr)?
    .run();
    tokio::select! {
        res = server => res,
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down file server...");
            Ok(())
        }
    }
}
