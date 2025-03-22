use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Participant {
        register(&self, new: CreateParticipant) -> (Participant, String);
        login(&self, req: LoginRequest) -> (Participant, String);
        find_by_id(&self, id: Ulid) -> Option<Participant>;
        get_by_id(&self, id: Ulid) -> Participant;
        update_by_id(&self, id: Ulid, update: ParticipantUpdate) -> Participant;
        change_password_by_id(&self, id: Ulid, req: PasswordChangeRequest) -> (Participant, String);
        delete_by_id(&self, id: Ulid) -> ();
    }
}
