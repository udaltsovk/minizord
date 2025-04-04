use std::sync::Arc;

use macros::implementation;

use super::{CreateParticipatedIn, ParticipatedIn, ParticipatedInUpdate};
use crate::{
    common::adapters::surrealdb::SurrealDB, tour::TourId, user::UserId,
};

implementation! {
    ParticipatedInRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateParticipatedIn) -> ParticipatedIn {
            self.db.0
                .create(new.get_id(Self::TABLE))
                .content(new.into_entity())
                .await?
                .expect("Failed to save ParticipatedIn object!")
        }

        find_all_by_in(&self, r#in: UserId, limit: u64, offset: u64) -> Vec<ParticipatedIn> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::record($in)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("in", r#in))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_in(&self, r#in: UserId) -> bool {
            !self.find_all_by_in(r#in, 1, 0).await?.is_empty()
        }

        find_all_by_out(&self, out: TourId, limit: u64, offset: u64) -> Vec<ParticipatedIn> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE out = type::record($out)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("table", Self::TABLE))
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
                .query(
                    r#"
                        SELECT * FROM type::table($table)
                            WHERE in = type::record($in) 
                                && out = type::record($out)
                            LIMIT 1
                    "#
                )
                .bind(("table", Self::TABLE))
                .bind(("in", r#in))
                .bind(("out", out))
                .await?
                .take(0)?
        }

        exists_by_in_and_out(&self, r#in: UserId, out: TourId) -> bool {
            self.find_by_in_and_out(r#in, out).await?.is_some()
        }

        update_by_in_and_out(&self, r#in: UserId, out: TourId, update: ParticipatedInUpdate) -> Option<ParticipatedIn> {
            self.db.0
                .update(Self::get_id(&r#in, &out))
                .merge(update)
                .await?
        }

        delete_by_in_and_out(&self, r#in: UserId, out: TourId) -> Option<ParticipatedIn> {
            self.db.0
                .delete(Self::get_id(&r#in, &out))
                .await?
        }
    }
}
