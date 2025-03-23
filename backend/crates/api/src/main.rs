use api::{Api, config, utils::lgtm::LGTM};
use repository::common::adapters::surrealdb::SurrealDB;

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

    // let minio = Minio::init(
    //     &config::MINIO_BASE_URL,
    //     &config::MINIO_USER,
    //     &config::MINIO_PASSWORD,
    //     &config::MINIO_BUCKET,
    // )
    // .await
    // .expect("Failed to init the file host");

    Api::setup(db).run().await
}
