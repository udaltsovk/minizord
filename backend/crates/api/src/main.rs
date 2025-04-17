use api::{Api, config};
use env_vars_config::set_env_only;
use utils::{
    adapters::{S3, SurrealDB},
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
        config::OTEL_ENDPOINT.clone(),
        config::OTEL_SERVICE_NAME.clone(),
    );

    config::test_values();

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

    let s3 = S3::init(
        &config::S3_BASE_URL,
        &config::S3_ACCESS_KEY,
        &config::S3_SECRET_KEY,
        &config::S3_REGION,
    )
    .await;

    Api::setup(lgtm.clone(), db, s3).run().await?;

    lgtm.shutdown().expect("Failed to shut down LGTM stuff");
    Ok(())
}
