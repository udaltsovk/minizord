use entity::technology::{
    CreateTechnology, Technology, TechnologyId, TechnologyUpdate,
};
use macros::{EntityId, implementation, surql_query};
use tracing::instrument;
use utils::adapters::{MobcPool, SurrealPool};

use super::{TechnologyRepository, TechnologyRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TechnologyRepository {
        pool: SurrealPool
    } as SurrealTechnologyRepository {
        #[instrument(skip_all, name = "TechnologyRepository::save")]
        async fn save(&self, new: CreateTechnology) -> Technology {
            let entity: Technology = new.into();
            self.pool
                .get()
                .await?
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        #[instrument(skip_all, name = "TechnologyRepository::find_by_id")]
        async fn find_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.pool
                .get()
                .await?
                .select(id.record_id())
                .await?
        }

        #[instrument(skip_all, name = "TechnologyRepository::exists_by_id")]
        async fn exists_by_id(&self, id: TechnologyId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        #[instrument(skip_all, name = "TechnologyRepository::find_by_name")]
        async fn find_by_name(&self, name: &str) -> Option<Technology> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/find_by_name"))
                .bind(("table", TechnologyId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "TechnologyRepository::exists_by_name")]
        async fn exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        #[instrument(skip_all, name = "TechnologyRepository::update_by_id")]
        async fn update_by_id(&self, id: TechnologyId, update: TechnologyUpdate) -> Option<Technology> {
            self.pool
                .get()
                .await?
                .update(id.record_id())
                .merge(update)
                .await?
        }

        #[instrument(skip_all, name = "TechnologyRepository::delete_by_id")]
        async fn delete_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.pool
                .get()
                .await?
                .delete(id.record_id())
                .await?
        }
    }
}
