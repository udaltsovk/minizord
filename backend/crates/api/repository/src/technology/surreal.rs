use std::sync::Arc;

use entity::technology::{
    CreateTechnology, Technology, TechnologyId, TechnologyUpdate,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{TechnologyRepository, TechnologyRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TechnologyRepository {
        db: Arc<SurrealDB>
    } as SurrealTechnologyRepository {
        async fn save(&self, new: CreateTechnology) -> Technology {
            let entity: Technology = new.into();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        async fn exists_by_id(&self, id: TechnologyId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn find_by_name(&self, name: &str) -> Option<Technology> {
            self.db.0
                .query(surql_query!("table/find_by_name"))
                .bind(("table", TechnologyId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        async fn update_by_id(&self, id: TechnologyId, update: TechnologyUpdate) -> Option<Technology> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        async fn delete_by_id(&self, id: TechnologyId) -> Option<Technology> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
