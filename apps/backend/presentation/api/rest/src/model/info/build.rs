use serde::Serialize;
use utoipa::ToSchema;

///
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct BuildInfo {
    ///
    #[schema(examples("2025-02-23 @ 17:00 (Europe/Moscow)"))]
    pub(super) comp_date: &'static str,

    ///
    #[schema(examples("c94e0b0a2ed3c3fc6fbcd2a93d682bc4adeb9924"))]
    pub(super) git_hash: &'static str,

    ///
    #[schema(examples("release"))]
    pub(super) profile: &'static str,
}
