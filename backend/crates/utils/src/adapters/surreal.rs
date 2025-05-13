use include_dir::Dir;
use mobc::{Connection, Manager, Pool, async_trait};
use mobc_surrealdb::{ConnectionProtocol, SurrealDBConnectionManager};
use surrealdb_migrations::MigrationRunner;

use super::mobc::MobcPool;

#[derive(Clone)]
pub struct SurrealPool(Pool<SurrealDBConnectionManager>);
impl SurrealPool {
    #[tracing::instrument(
        name = "SurrealPool::setup",
        skip_all,
        level = "debug"
    )]
    #[tracing::instrument(
        name = "SurrealPool::init",
        skip_all,
        level = "debug"
    )]
    pub async fn init(
        address: &'static str,
        namespace: &'static str,
        database: &'static str,
        username: &'static str,
        password: &'static str,
        secure: bool,
        max_open: u64,
    ) -> Self {
        Self(Self::new_pool(
            max_open,
            SurrealDBConnectionManager::new_with_protocol(
                if secure {
                    ConnectionProtocol::Wss
                } else {
                    ConnectionProtocol::Ws
                },
                address,
                username,
                password,
                Some(namespace),
                Some(database),
            ),
        ))
    }

    #[tracing::instrument(
        name = "SurrealPool::migrate",
        skip_all,
        level = "debug"
    )]
    pub async fn migrate(self, dir: &Dir<'static>) -> Result<Self, ()> {
        tracing::trace!("Running database migrations");
        MigrationRunner::new(&*self.get().await.map_err(|_| ())?)
            .load_files(dir)
            .up()
            .await
            .map_err(|_| ())?;

        Ok(self)
    }
}
#[async_trait]
impl MobcPool<SurrealDBConnectionManager> for SurrealPool {
    #[tracing::instrument(name = "SurrealPool::get", skip_all, level = "debug")]
    async fn get(
        &self,
    ) -> Result<
        Connection<SurrealDBConnectionManager>,
        mobc::Error<<SurrealDBConnectionManager as Manager>::Error>,
    > {
        self.0.get().await
    }
}
