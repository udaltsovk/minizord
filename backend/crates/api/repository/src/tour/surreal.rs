use entity::tour::{CreateTour, Tour, TourId, TourUpdate};
use macros::{EntityId, implementation, surql_query};
use tracing::instrument;
use utils::adapters::SurrealDB;

use super::{TourRepository, TourRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TourRepository {
        db: SurrealDB
    } as SurrealTourRepository {
        #[instrument(skip_all, name = "TourRepository::save")]
        async fn save(&self, new: CreateTour) -> Tour {
            let entity: Tour = new.into();
            self.db
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        #[instrument(skip_all, name = "TourRepository::find_by_id")]
        async fn find_by_id(&self, id: TourId) -> Option<Tour> {
            self.db
                .select(id.record_id())
                .await?
        }

        #[instrument(skip_all, name = "TourRepository::exists_by_id")]
        async fn exists_by_id(&self, id: TourId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        #[instrument(skip_all, name = "TourRepository::find_by_name")]
        async fn find_by_name(&self, name: &str) -> Option<Tour> {
            self.db
                .query(surql_query!("table/find_by_name"))
                .bind(("table", TourId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        #[instrument(skip_all, name = "TourRepository::exists_by_name")]
        async fn exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        #[instrument(skip_all, name = "TourRepository::update_by_id")]
        async fn update_by_id(&self, id: TourId, update: TourUpdate) -> Option<Tour> {
            self.db
                .update(id.record_id())
                .merge(update)
                .await?
        }

        #[instrument(skip_all, name = "TourRepository::delete_by_id")]
        async fn delete_by_id(&self, id: TourId) -> Option<Tour> {
            self.db
                .delete(id.record_id())
                .await?
        }
    }
}
