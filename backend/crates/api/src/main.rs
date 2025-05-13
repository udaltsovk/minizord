use api::{Api, config};
use env_vars_config::set_env_only;
use include_dir::include_dir;
use utils::{
    adapters::{S3, SurrealPool},
    lgtm::LGTM,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        use config::OTEL_SERVICE_NAME;
        set_env_only!(OTEL_SERVICE_NAME);
    }
    config::init();

    let lgtm = LGTM::init(
        &config::OTEL_ENDPOINT,
        &config::OTEL_SERVICE_NAME,
        &config::METRICS_ADDRESS,
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
