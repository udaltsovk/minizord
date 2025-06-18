use entity::{
    EntityId,
    specialization::{
        CreateSpecialization, Specialization, SpecializationId,
        SpecializationUpdate,
    },
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{SpecializationRepository, SpecializationRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = SpecializationRepositoryResult)]
pub mod repository {
    struct SurrealSpecializationRepository {
        pool: SurrealPool,
    }

    impl SpecializationRepository for SurrealSpecializationRepository {
        async fn save(&self, new: CreateSpecialization) -> Specialization {
            let entity: Specialization = new.into();
            self.pool
                .get()
                .await?
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(
            &self,
            id: SpecializationId,
        ) -> Option<Specialization> {
            self.pool.get().await?.select(id.record_id()).await?
        }

        async fn exists_by_id(&self, id: SpecializationId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn find_by_name(&self, name: &str) -> Option<Specialization> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/find_by_name"))
                .bind(("table", SpecializationId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        async fn update_by_id(
            &self,
            id: SpecializationId,
            update: SpecializationUpdate,
        ) -> Option<Specialization> {
            self.pool
                .get()
                .await?
                .update(id.record_id())
                .merge(update)
                .await?
        }

        async fn delete_by_id(
            &self,
            id: SpecializationId,
        ) -> Option<Specialization> {
            self.pool.get().await?.delete(id.record_id()).await?
        }
    }
}
