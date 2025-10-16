use std::collections::HashMap;

use application::repository::user::UserRepository;
use async_trait::async_trait;
use domain::user::User;
use lib::{domain::Id, instrument_all};
use surrealdb::Value;
use tap::{Conv as _, Pipe as _};

use crate::{
    entity::{SurrealId, SurrealIdWithTable as _, user::StoredUser},
    error::SurrealAdapterError,
    mobc_pool::MobcPool as _,
    repository::{ExtractValue as _, SurrealRepositoryImpl},
    surql_query,
};

#[async_trait]
#[instrument_all("SurrealUserRepository")]
impl UserRepository for SurrealRepositoryImpl<User> {
    type AdapterError = SurrealAdapterError;

    async fn save(&self, source: User) -> Result<User, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .create(SurrealId::<User>::TABLE)
            .content(StoredUser::from(source))
            .await?
            .conv::<Option<StoredUser>>()
            .map(User::from)
            .expect("idk how this can be none")
            .pipe(Ok)
    }

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .select(id.conv::<SurrealId<_>>())
            .await?
            .conv::<Option<StoredUser>>()
            .map(User::from)
            .pipe(Ok)
    }

    async fn exists_by_id(
        &self,
        id: Id<User>,
    ) -> Result<bool, Self::AdapterError> {
        self.find_by_id(id).await?.is_some().pipe(Ok)
    }

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .query(surql_query!("table/find_by_field"))
            .bind(("table", SurrealId::<User>::TABLE))
            .bind(("field", "email"))
            .bind(("value", email.to_string()))
            .await?
            .take::<Option<StoredUser>>(0)?
            .map(User::from)
            .pipe(Ok)
    }

    async fn exists_by_email(
        &self,
        email: &str,
    ) -> Result<bool, Self::AdapterError> {
        self.find_by_email(email).await?.is_some().pipe(Ok)
    }

    async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .query(surql_query!("table/find_by_field"))
            .bind(("table", SurrealId::<User>::TABLE))
            .bind(("field", "username"))
            .bind(("value", username.to_string()))
            .await?
            .take::<Option<StoredUser>>(0)?
            .map(User::from)
            .pipe(Ok)
    }

    async fn exists_by_username(
        &self,
        username: &str,
    ) -> Result<bool, Self::AdapterError> {
        self.find_by_username(username).await?.is_some().pipe(Ok)
    }

    async fn delete_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .delete(id.conv::<SurrealId<_>>())
            .await?
            .conv::<Option<StoredUser>>()
            .map(User::from)
            .pipe(Ok)
    }

    async fn count_by_role(
        &self,
    ) -> Result<HashMap<String, u32>, Self::AdapterError> {
        self.pool
            .get()
            .await?
            .query(surql_query!("table/count_by_field"))
            .bind(("table", SurrealId::<User>::TABLE))
            .bind(("field", "role"))
            .await?
            .take::<Value>(0)?
            .extract::<HashMap<_, _>>()
            .pipe(Ok)
    }
}
