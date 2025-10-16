use std::sync::OnceLock;

use serde::Serialize;
use utoipa::ToSchema;

use crate::model::info::build::BuildInfo;

mod build;

///
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ApiInfo {
    ///
    #[schema(examples("minizord-api"))]
    name: &'static str,

    ///
    #[schema(examples("0.1.0"))]
    version: &'static str,

    ///
    #[schema(format = Uri)]
    documentation: String,

    ///
    #[schema(examples("Welcome, traveler!"))]
    about: &'static str,

    ///
    build_info: BuildInfo,
}

static APIINFO: OnceLock<ApiInfo> = OnceLock::new();

impl ApiInfo {
    pub fn get(base_api_url: &str) -> &Self {
        APIINFO.get_or_init(|| Self {
            name: "minizord-api",
            version: env!("CARGO_PKG_VERSION"),
            documentation: format!("{base_api_url}/openapi"),
            about: "Welcome, traveler!",
            build_info: BuildInfo {
                comp_date: env!("COMPILATION_DATE"),
                git_hash: env!("GIT_HASH", "unknown"),
                profile: env!("COMPILATION_PROFILE"),
            },
        })
    }
}
