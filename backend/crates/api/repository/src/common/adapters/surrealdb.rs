use include_dir::include_dir;
use surrealdb::{
    Result, Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use surrealdb_migrations::MigrationRunner;

#[derive(Clone)]
pub struct SurrealDB(pub Surreal<Client>);

impl SurrealDB {
    #[tracing::instrument(skip_all, level = "debug")]
    async fn setup(
        self,
        address: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self> {
        self.0.connect::<Ws>(address).await?;
        tracing::trace!("Connected to the database");

        self.0.signin(Root { username, password }).await?;
        tracing::trace!("Signed in the database");

        self.0.use_ns(namespace).use_db(database).await?;
        tracing::trace!("Switched to the namespace and database");
        Ok(self)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub async fn init(
        address: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Self {
        let surreal: Surreal<Client> = Surreal::init();
        let db = Self(surreal);
        db.setup(address, username, password, namespace, database)
            .await
            .expect("Failed to init the database")
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub async fn migrate(self) -> Self {
        MigrationRunner::new(&self.0)
            .load_files(&include_dir!("crates/api/repository/db/surreal"))
            .up()
            .await
            .expect("Failed to run migrations");

        self
    }
}
