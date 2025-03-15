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
    async fn setup(
        self,
        address: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self> {
        self.0.connect::<Ws>(address).await?;
        self.0.signin(Root { username, password }).await?;

        self.0.use_ns(namespace).use_db(database).await?;
        Ok(self)
    }

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

    pub async fn migrate(self) -> Self {
        MigrationRunner::new(&self.0)
            .load_files(&include_dir!("crates/api/repository/db/surreal"))
            .up()
            .await
            .expect("Failed to run migrations");

        self
    }
}
