use actix_web::{
    App, HttpServer,
    middleware::{Compress, TrailingSlash},
};
use actix_web_lab::middleware::{CatchPanic, NormalizePath};
use api::{
    app_setup, config,
    utils::{logger::CustomLogger, openapi::OpenApiVisualiser},
};
use env_logger::Env;
use repository::common::adapters::surrealdb::SurrealDB;
use utoipa_actix_web::AppExt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let default_log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    env_logger::init_from_env(Env::default().default_filter_or(default_log_level));

    let db = SurrealDB::init(
        &config::DB_ADDRESS,
        &config::DB_NAMESPACE,
        &config::DB_NAME,
        &config::DB_USER,
        &config::DB_PASSWORD,
    )
    .await
    .migrate()
    .await;

    // let minio = Minio::init(
    //     &config::MINIO_BASE_URL,
    //     &config::MINIO_USER,
    //     &config::MINIO_PASSWORD,
    //     &config::MINIO_BUCKET,
    // )
    // .await
    // .expect("Failed to init the file host");

    let app_config = app_setup(db);

    HttpServer::new(move || {
        App::new()
            .wrap(CatchPanic::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(if cfg!(feature = "swagger") {
                TrailingSlash::MergeOnly
            } else {
                TrailingSlash::Trim
            }))
            .wrap(CustomLogger::new())
            .into_utoipa_app()
            .openapi(app_config.openapi.clone())
            .configure(app_config.clone().build())
            .openapi_service(OpenApiVisualiser::service)
            .into_app()
    })
    .bind(config::SERVER_ADDRESS.clone())?
    .run()
    .await
}
