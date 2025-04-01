use std::sync::Arc;

use macros::{RepositoryId, implementation};
use ulid::Ulid;

use super::{
    CreateSpecialization, Specialization, SpecializationId,
    SpecializationUpdate,
};
use crate::common::adapters::surrealdb::SurrealDB;

impl From<SpecializationId> for Ulid {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(id: SpecializationId) -> Self {
        Self::from_string(&id.to_string()).unwrap()
    }
}

implementation! {
    SpecializationRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateSpecialization) -> Specialization {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .unwrap()
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
                .query(
                    r#"
                        SELECT * FROM type::table($specialization_table)
                            WHERE name = type::string($name)
                            LIMIT 1
                    "#
                )
                .bind(("specialization_table", SpecializationId::TABLE))
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
