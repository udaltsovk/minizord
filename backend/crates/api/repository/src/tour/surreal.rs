use std::sync::Arc;

use entity::tour::{CreateTour, Tour, TourId, TourUpdate};
use macros::{EntityId, implementation};
use utils::adapters::SurrealDB;

use super::{TourRepository, TourRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TourRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateTour) -> Tour {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_by_id(&self, id: TourId) -> Option<Tour> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: TourId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_name(&self, name: &str) -> Option<Tour> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($tour_table)
                            WHERE name = type::string($name)
                            LIMIT 1
                    "#
                )
                .bind(("tour_table", TourId::TABLE))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        exists_by_name(&self, name: &str) -> bool {
            self.find_by_name(name).await?.is_some()
        }

        update_by_id(&self, id: TourId, update: TourUpdate) -> Option<Tour> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: TourId) -> Option<Tour> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
