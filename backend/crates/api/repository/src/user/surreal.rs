use std::collections::HashMap;

use entity::{
    EntityId,
    user::{CreateUser, User, UserId, UserUpdate},
};
use macros::{implementation, surql_query};
use surrealdb::Value;
use utils::adapters::{MobcPool, SurrealPool};

use super::{UserRepository, UserRepositoryResult};
use crate::common::{ExtractValue as _, RepositoryError};

#[implementation(result = UserRepositoryResult)]
pub mod repository {
    struct SurrealUserRepository {
        pool: SurrealPool,
    }

    impl UserRepository for SurrealUserRepository {
        async fn save(&self, new: CreateUser) -> User {
            let entity: User = new.into();
            self.pool
                .get()
                .await?
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: UserId) -> Option<User> {
            self.pool.get().await?.select(id.record_id()).await?
        }

        async fn exists_by_id(&self, id: UserId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn find_by_email(&self, email: &str) -> Option<User> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/user/find_by_email"))
                .bind(("table", UserId::TABLE))
                .bind(("email", email.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_email(&self, email: &str) -> bool {
            self.find_by_email(email).await?.is_some()
        }

        async fn find_by_username(&self, username: &str) -> Option<User> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/user/find_by_username"))
                .bind(("table", UserId::TABLE))
                .bind(("username", username.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_username(&self, username: &str) -> bool {
            self.find_by_username(username).await?.is_some()
        }

        async fn update_by_id(
            &self,
            id: UserId,
            update: UserUpdate,
        ) -> Option<User> {
            self.pool
                .get()
                .await?
                .update(id.record_id())
                .merge(update)
                .await?
        }

        async fn delete_by_id(&self, id: UserId) -> Option<User> {
            self.pool.get().await?.delete(id.record_id()).await?
        }

        async fn count_by_role(&self) -> HashMap<String, u32> {
            self.pool
                .get()
                .await?
                .query(surql_query!("table/count_by_field"))
                .bind(("table", UserId::TABLE))
                .bind(("field", "role"))
                .await?
                .take::<Value>(0)?
                .extract()
        }
    }
}
