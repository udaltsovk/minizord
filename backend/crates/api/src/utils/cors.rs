use actix_cors::Cors;
use actix_web::http::header;

use crate::config;

#[tracing::instrument(level = "trace")]
pub fn default_cors() -> Cors {
    if config::DEPLOY_DOMAIN.as_str() == "localhost" {
        Cors::permissive()
    } else {
        Cors::default()
            .allowed_origin(&format!(
                "https://{}",
                config::DEPLOY_DOMAIN.clone()
            ))
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ACCEPT_ENCODING,
            ])
            .supports_credentials()
            .block_on_origin_mismatch(true)
    }
}
