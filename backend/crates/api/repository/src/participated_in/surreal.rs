use entity::{
    EntityId,
    participated_in::{
        CreateParticipatedIn, ParticipatedIn, ParticipatedInId,
        ParticipatedInUpdate,
    },
    tour::TourId,
    user::UserId,
};
use macros::{implementation, surql_query};
use utils::adapters::{MobcPool, SurrealPool};

use super::{ParticipatedInRepository, ParticipatedInRepositoryResult};
use crate::common::RepositoryError;

#[implementation(result = ParticipatedInRepositoryResult)]
pub mod repository {
    struct SurrealParticipatedInRepository {
        pool: SurrealPool,
    }

    impl ParticipatedInRepository for SurrealParticipatedInRepository {
        async fn save(&self, new: CreateParticipatedIn) -> ParticipatedIn {
            self.pool
                .get()
                .await?
                .create(new.get_id().record_id())
                .content(ParticipatedIn::from(new))
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_all_by_in(
            &self,
            r#in: UserId,
            limit: u16,
            offset: u64,
        ) -> Vec<ParticipatedIn> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_in"))
                .bind(("table", ParticipatedInId::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        async fn find_all_by_out(
            &self,
            out: TourId,
            limit: u16,
            offset: u64,
        ) -> Vec<ParticipatedIn> {
            self.pool
                .get()
                .await?
                .query(surql_query!("relation/find_all_by_out"))
                .bind(("table", ParticipatedInId::TABLE))
                .bind(("out", out))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_out(&self, out: TourId) -> bool {
            !self.find_all_by_out(out, 1, 0).await?.is_empty()
        }

        async fn find_by_in_and_out(
            &self,
            r#in: UserId,
            out: TourId,
        ) -> Option<ParticipatedIn> {
            self.pool
                .get()
                .await?
                .select(self.get_id(&r#in, &out))
                .await?
        }

        async fn exists_by_in_and_out(
            &self,
            r#in: UserId,
            out: TourId,
        ) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        async fn update_by_in_and_out(
            &self,
            r#in: UserId,
            out: TourId,
            update: ParticipatedInUpdate,
        ) -> Option<ParticipatedIn> {
            self.pool
                .get()
                .await?
                .update(self.get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        async fn delete_by_in_and_out(
            &self,
            r#in: UserId,
            out: TourId,
        ) -> Option<ParticipatedIn> {
            self.pool
                .get()
                .await?
                .delete(self.get_id(&r#in, &out))
                .await?
        }
    }
}
