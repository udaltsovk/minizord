use aws_sdk_s3::{
    Client, Config,
    config::{Credentials, Region},
};

#[derive(Clone)]
pub struct S3(pub Client);

impl S3 {
    #[tracing::instrument(name = "S3::init", skip_all, level = "debug")]
    pub async fn init(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        region: &str,
    ) -> Self {
        let config = Config::builder()
            .endpoint_url(endpoint)
            .force_path_style(true)
            .region(Region::new(region.to_string()))
            .credentials_provider(Credentials::new(
                access_key, secret_key, None, None, "static",
            ))
            .build();

        Self(Client::from_conf(config))
    }

    #[tracing::instrument(
        name = "S3::create_bucket_if_not_exists",
        skip_all,
        level = "debug"
    )]
    async fn create_bucket_if_not_exists(&self, bucket: &str) {
        tracing::trace!("Checking if bucket named `{bucket}` exists");
        if self.0.head_bucket().bucket(bucket).send().await.is_err() {
            tracing::trace!("Creating bucket named `{bucket}`");
            self.0
                .create_bucket()
                .bucket(bucket)
                .send()
                .await
                .expect("Failed to create bucket");
        }
    }
}
