use entity::{
    EntityId,
    tour::{CreateTour, Tour, TourId, TourUpdate},
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{TourRepository, TourRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = TourRepositoryResult)]
pub mod repository {
    struct SurrealTourRepository {
        pool: SurrealPool,
    }

    impl TourRepository for SurrealTourRepository {
        async fn save(&self, new: CreateTour) -> Tour {
            let entity: Tour = new.into();
            self.pool
                .get()
                .await?
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: TourId) -> Option<Tour> {
            self.pool.get().await?.select(id.record_id()).await?
        }

        async fn exists_by_id(&self, id: TourId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn find_by_name(&self, name: &str) -> Option<Tour> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/find_by_name"))
                .bind(("table", TourId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        async fn update_by_id(
            &self,
            id: TourId,
            update: TourUpdate,
        ) -> Option<Tour> {
            self.pool
                .get()
                .await?
                .update(id.record_id())
                .merge(update)
                .await?
        }

        async fn delete_by_id(&self, id: TourId) -> Option<Tour> {
            self.pool.get().await?.delete(id.record_id()).await?
        }
    }
}
