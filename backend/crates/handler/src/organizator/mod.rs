use macros::{handler, response};

pub mod implementation;

handler! {
    Organizator {

    }
}

response! {
    OrganizatorAuth {
        token: String,
        organizator: Organizator
    }
}
