use std::sync::Arc;

use entity::technology::{
    CreateTechnology, Technology, TechnologyId, TechnologyUpdate,
};
use macros::{EntityId, implementation};
use utils::adapters::SurrealDB;

use super::{TechnologyRepository, TechnologyRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TechnologyRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateTechnology) -> Technology {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: TechnologyId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_name(&self, name: &str) -> Option<Technology> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/table/find_by_name.surql"))
                .bind(("table", TechnologyId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        update_by_id(&self, id: TechnologyId, update: TechnologyUpdate) -> Option<Technology> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
