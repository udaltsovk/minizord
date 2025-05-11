use std::sync::Arc;

use entity::user::{CreateUser, User, UserId, UserUpdate};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{UserRepository, UserRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    UserRepository {
        db: Arc<SurrealDB>
    } as SurrealUserRepository {
        save(&self, new: CreateUser) -> User {
            let entity: User = new.into();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_by_id(&self, id: UserId) -> Option<User> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: UserId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_email(&self, email: &str) -> Option<User> {
            self.db.0
                .query(surql_query!("table/user/find_by_email"))
                .bind(("table", UserId::TABLE))
                .bind(("email", email.to_string()))
                .await?
                .take(0)?
        }

        exists_by_email(&self, email: &str) -> bool {
            self.find_by_email(email).await?.is_some()
        }

        find_by_username(&self, username: &str) -> Option<User> {
            self.db.0
                .query(surql_query!("table/user/find_by_username"))
                .bind(("table", UserId::TABLE))
                .bind(("username", username.to_string()))
                .await?
                .take(0)?
        }

        exists_by_username(&self, username: &str) -> bool {
            self.find_by_username(username).await?.is_some()
        }

        update_by_id(&self, id: UserId, update: UserUpdate) -> Option<User> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: UserId) -> Option<User> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
