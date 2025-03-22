use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    mentor::{CreateMentor, Mentor, MentorUpdate},
};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Mentor {
        register(&self, new: CreateMentor) -> (Mentor, String);
        login(&self, req: LoginRequest) -> (Mentor, String);
        find_by_id(&self, id: Ulid) -> Option<Mentor>;
        get_by_id(&self, id: Ulid) -> Mentor;
        update_by_id(&self, id: Ulid, update: MentorUpdate) -> Mentor;
        change_password_by_id(&self, id: Ulid, req: PasswordChangeRequest) -> (Mentor, String);
        delete_by_id(&self, id: Ulid) -> ();
    }
}
