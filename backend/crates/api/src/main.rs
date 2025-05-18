use api::{Api, config};
use include_dir::include_dir;
use utils::{
    LGTM,
    adapters::{S3, SurrealPool},
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let lgtm = LGTM::init(
        &config::OTEL_ENDPOINT,
        &config::METRICS_ADDRESS,
        "minizord",
        "api",
    );

    config::test_values();

    let pool = SurrealPool::init(
        &config::DB_ADDRESS,
        &config::DB_NAMESPACE,
        &config::DB_NAME,
        &config::DB_USER,
        &config::DB_PASSWORD,
        false,
        *config::DB_MAX_POOL_SIZE,
    )
    .await
    .migrate(&include_dir!("crates/api/repository/db/surreal"))
    .await
    .expect("Failed to run migrations");

    let s3 = S3::init(
        &config::S3_BASE_URL,
        &config::S3_ACCESS_KEY,
        &config::S3_SECRET_KEY,
        &config::S3_REGION,
    )
    .await;

    Api::setup(lgtm.clone(), pool, s3).await.run().await?;

    lgtm.shutdown().expect("Failed to shut down LGTM stuff");
    Ok(())
}
