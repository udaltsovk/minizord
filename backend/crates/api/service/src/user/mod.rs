use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    user::{CreateUser, User, UserUpdate},
};
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    User
        Err: ServiceError
    {
        async fn register(&self, new: CreateUser) -> (User, String);
        async fn login(&self, req: LoginRequest) -> (User, String);
        async fn find_by_id(&self, id: Ulid) -> Option<User>;
        async fn get_by_id(&self, id: Ulid) -> User;
        async fn update_by_id(&self, id: Ulid, update: UserUpdate, is_self: bool) -> User;
        async fn change_password_by_id(&self, id: Ulid, req: PasswordChangeRequest, is_self: bool) -> (User, String);
        async fn delete_by_id(&self, id: Ulid) -> ();
    }
}
