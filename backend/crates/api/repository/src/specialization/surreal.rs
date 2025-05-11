use std::sync::Arc;

use entity::specialization::{
    CreateSpecialization, Specialization, SpecializationId,
    SpecializationUpdate,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{SpecializationRepository, SpecializationRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    SpecializationRepository {
        db: Arc<SurrealDB>
    } as SurrealSpecializationRepository {
        save(&self, new: CreateSpecialization) -> Specialization {
            let entity: Specialization = new.into();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_by_id(&self, id: SpecializationId) -> Option<Specialization> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: SpecializationId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_name(&self, name: &str) -> Option<Specialization> {
            self.db.0
                .query(surql_query!("table/find_by_name"))
                .bind(("table", SpecializationId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        update_by_id(&self, id: SpecializationId, update: SpecializationUpdate) -> Option<Specialization> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: SpecializationId) -> Option<Specialization> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
