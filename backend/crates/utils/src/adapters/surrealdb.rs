use std::{ops::Deref, sync::Arc};

use include_dir::Dir;
use surrealdb::{
    Result, Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use surrealdb_migrations::MigrationRunner;

#[derive(Clone)]
pub struct SurrealDB(Arc<Surreal<Client>>);
impl SurrealDB {
    #[tracing::instrument(name = "SurrealDB::setup", skip_all, level = "debug")]
    async fn setup(
        self,
        address: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self> {
        tracing::trace!("Connecting to the database instance");
        self.0.connect::<Ws>(address).await?;

        tracing::trace!("Signing in the database instance");
        self.0
            .signin(Root {
                username,
                password,
            })
            .await?;

        tracing::trace!("Switching to the provided namespace and database");
        self.0.use_ns(namespace).use_db(database).await?;
        Ok(self)
    }

    #[tracing::instrument(name = "SurrealDB::init", skip_all, level = "debug")]
    pub async fn init(
        address: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Self {
        let db = Self(Arc::new(Surreal::init()));
        db.setup(address, username, password, namespace, database)
            .await
            .expect("Failed to init the database")
    }

    #[tracing::instrument(
        name = "SurrealDB::migrate",
        skip_all,
        level = "debug"
    )]
    pub async fn migrate(self, dir: &Dir<'static>) -> Self {
        tracing::trace!("Running database migrations");
        MigrationRunner::new(&self.0)
            .load_files(dir)
            .up()
            .await
            .expect("Failed to run migrations");

        self
    }
}
impl Deref for SurrealDB {
    type Target = Arc<Surreal<Client>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
