use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use domain::user::User;
use lib::domain::Id;

#[async_trait]
pub trait UserRepository {
    type AdapterError: Debug + Send + Sync;

    async fn save(&self, source: User) -> Result<User, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn exists_by_id(
        &self,
        id: Id<User>,
    ) -> Result<bool, Self::AdapterError>;

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn exists_by_email(
        &self,
        email: &str,
    ) -> Result<bool, Self::AdapterError>;

    async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn exists_by_username(
        &self,
        username: &str,
    ) -> Result<bool, Self::AdapterError>;

    async fn delete_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn count_by_role(
        &self,
    ) -> Result<HashMap<String, u32>, Self::AdapterError>;
}
