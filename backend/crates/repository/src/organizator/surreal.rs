use super::OrganizatorId;
use macros::implementation;
use ulid::Ulid;

impl Into<Ulid> for OrganizatorId {
    fn into(self) -> Ulid {
        Ulid::from_string(&self.to_string()).unwrap()
    }
}

implementation! {
    OrganizatorRepository as Surreal {

    }
}
