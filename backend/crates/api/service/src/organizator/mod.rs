use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    organizator::{CreateOrganizator, Organizator, OrganizatorUpdate},
};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Organizator {
        register(&self, new: CreateOrganizator) -> (Organizator, String);
        login(&self, req: LoginRequest) -> (Organizator, String);
        find_by_id(&self, id: Ulid) -> Option<Organizator>;
        get_by_id(&self, id: Ulid) -> Organizator;
        update_by_id(&self, id: Ulid, update: OrganizatorUpdate) -> Organizator;
        change_password_by_id(&self, id: Ulid, req: PasswordChangeRequest) -> (Organizator, String);
        delete_by_id(&self, id: Ulid) -> ();
    }
}
