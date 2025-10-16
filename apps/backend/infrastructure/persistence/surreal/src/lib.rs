#![feature(const_type_name)]
use include_dir::Dir;
use lib::instrument_all;
use mobc::{Connection, Manager, Pool, async_trait};
pub use mobc_surrealdb::ConnectionProtocol;
use mobc_surrealdb::SurrealDBConnectionManager;
use surrealdb_migrations::MigrationRunner;
use tap::Pipe as _;

use crate::mobc_pool::MobcPool;

pub mod entity;
pub mod error;
mod mobc_pool;
pub mod repository;

#[derive(Clone)]
pub struct Surreal(Pool<SurrealDBConnectionManager>);

#[async_trait]
impl MobcPool<SurrealDBConnectionManager> for Surreal {
    #[tracing::instrument(name = "Surreal::get", skip_all, level = "debug")]
    async fn get(
        &self,
    ) -> Result<
        Connection<SurrealDBConnectionManager>,
        mobc::Error<<SurrealDBConnectionManager as Manager>::Error>,
    > {
        self.0.get().await
    }
}

#[instrument_all("Surreal")]
impl Surreal {
    pub async fn new(
        address: &'static str,
        namespace: &'static str,
        database: &'static str,
        username: &'static str,
        password: &'static str,
        protocol: ConnectionProtocol,
        max_open: u64,
    ) -> Self {
        Self::new_pool(
            max_open,
            SurrealDBConnectionManager::new_with_protocol(
                protocol,
                address,
                username,
                password,
                Some(namespace),
                Some(database),
            ),
        )
        .pipe(Self)
    }

    pub async fn migrate(self, dir: &Dir<'static>) -> Result<Self, String> {
        tracing::trace!("Running database migrations");
        MigrationRunner::new(
            &*self.get().await.map_err(|err| err.to_string())?,
        )
        .load_files(dir)
        .up()
        .await
        .map_err(|err| err.to_string())?;

        Ok(self)
    }
}
