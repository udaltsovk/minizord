use utoipa_axum::{router::OpenApiRouter, routes};

use crate::module::ModulesExt;

mod register;

pub const USERS_TAG: &str = "users";

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new().routes(routes!(register::register::<M>))
}
