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
        register(&self, new: CreateUser) -> (User, String);
        login(&self, req: LoginRequest) -> (User, String);
        find_by_id(&self, id: Ulid) -> Option<User>;
        get_by_id(&self, id: Ulid) -> User;
        update_by_id(&self, id: Ulid, update: UserUpdate, is_self: bool) -> User;
        change_password_by_id(&self, id: Ulid, req: PasswordChangeRequest, is_self: bool) -> (User, String);
        delete_by_id(&self, id: Ulid) -> ();
    }
}
