use std::sync::Arc;

use entity::{
    participated_in::{
        CreateParticipatedIn, ParticipatedIn, ParticipatedInId,
        ParticipatedInUpdate,
    },
    tour::TourId,
    user::UserId,
};
use macros::{EntityId, implementation};
use utils::adapters::SurrealDB;

use super::{ParticipatedInRepository, ParticipatedInRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    ParticipatedInRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateParticipatedIn) -> ParticipatedIn {
            self.db.0
                .create(new.get_id().record_id())
                .content(new.into_entity())
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_all_by_in(&self, r#in: UserId, limit: u16, offset: u64) -> Vec<ParticipatedIn> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/relation/find_all_by_in.surql"))
                .bind(("table", ParticipatedInId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: TourId, limit: u16, offset: u64) -> Vec<ParticipatedIn> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/relation/find_all_by_out.surql"))
                .bind(("table", ParticipatedInId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_out(&self, out: TourId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        find_by_in_and_out(&self, r#in: UserId, out: TourId) -> Option<ParticipatedIn> {
            self.db.0
                .select(self.get_id(&r#in, &out))
                .await?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: TourId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        update_by_in_and_out(&self, r#in: UserId, out: TourId, update: ParticipatedInUpdate) -> Option<ParticipatedIn> {
            self.db.0
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: TourId) -> Option<ParticipatedIn> {
            self.db.0
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
