use std::sync::Arc;

use entity::{
    has_experience_as::{
        HasExperienceAs, HasExperienceAsId, UpsertHasExperienceAs,
    },
    specialization::SpecializationId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{HasExperienceAsRepository, HasExperienceAsRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    HasExperienceAsRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        upsert_by_in_and_out(&self, r#in: UserId, out: SpecializationId, object: UpsertHasExperienceAs) -> HasExperienceAs {
            let result: Option<HasExperienceAs> = self.db.0
                .query(surql_query!("relation/upsert_by_in_and_out"))
                .bind(("in", r#in))
                .bind(("id", object.get_id().record_id()))
                .bind(("out", out))
                .bind(("object", object))
                .await?
                .take(0)?;
            result.ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<HasExperienceAs> {
            self.db.0
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", HasExperienceAsId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: SpecializationId, limit: u16, offset: u64) -> Vec<HasExperienceAs> {
            self.db.0
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", HasExperienceAsId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_out(&self, out: SpecializationId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> Option<HasExperienceAs> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        delete_by_in_and_out(&self, r#in: UserId, out: SpecializationId) -> Option<HasExperienceAs> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
