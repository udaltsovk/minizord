use api::{Api, config, utils::lgtm::LGTM};
use repository::common::adapters::{s3::S3, surrealdb::SurrealDB};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let _lgtm = LGTM::init();

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

    Api::setup(db, s3).run().await
}
