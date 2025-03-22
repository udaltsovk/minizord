use actix_web::{
    App, HttpServer,
    middleware::{Compress, TrailingSlash},
};
use actix_web_lab::middleware::{CatchPanic, NormalizePath};
use api::{
    app_setup, config,
    utils::{lgtm::LGTM, openapi::OpenApiVisualiser},
};
use repository::common::adapters::surrealdb::SurrealDB;
use utoipa_actix_web::AppExt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    LGTM::init_logging();

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

    tracing::info!("Starting the web server");

    HttpServer::new(move || {
        App::new()
            .wrap(LGTM::tracing())
            .wrap(LGTM::metrics())
            .wrap(CatchPanic::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(if cfg!(feature = "swagger") {
                TrailingSlash::MergeOnly
            } else {
                TrailingSlash::Trim
            }))
            .into_utoipa_app()
            .openapi(app_config.openapi.clone())
            .configure(app_config.clone().build())
            .openapi_service(OpenApiVisualiser::service)
            .into_app()
    })
    .bind(config::SERVER_ADDRESS.clone())?
    .run()
    .await?;

    tracing::info!("Shutting down web server");
    Ok(())
}
