use std::sync::Arc;

use macros::implementation;
use ulid::Ulid;

use super::{CreateOrganizator, Organizator, OrganizatorId, OrganizatorUpdate};
use crate::common::adapters::surrealdb::SurrealDB;

impl Into<Ulid> for OrganizatorId {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into(self) -> Ulid {
        Ulid::from_string(&self.to_string()).unwrap()
    }
}

implementation! {
    OrganizatorRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateOrganizator) -> Organizator {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .unwrap()
        }

        find_by_id(&self, id: OrganizatorId) -> Option<Organizator> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: OrganizatorId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_username(&self, username: &str) -> Option<Organizator> {
            self.db.0
                .query(
                    r#"
                        SELECT * FROM type::table($organizator_table)
                            WHERE username = type::string($username)
                            LIMIT 1
                    "#
                )
                .bind(("organizator_table", OrganizatorId::TABLE))
                .bind(("username", username.to_string()))
                .await?
                .take(0)?
        }

        exists_by_username(&self, username: &str) -> bool {
            self.find_by_username(username).await?.is_some()
        }

        update_by_id(&self, id: OrganizatorId, update: OrganizatorUpdate) -> Option<Organizator> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: OrganizatorId) -> Option<Organizator> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
