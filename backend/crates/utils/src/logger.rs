use actix_contrib_logger::middleware::Logger;
use actix_web::{HttpMessage as _, http::StatusCode};
use tracing::log::Level;
use tracing_actix_web::RequestId;

pub struct CustomActixLogger;
impl CustomActixLogger {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Logger {
        Logger::new("[%{request_id}xi] \"%r\" %s (took %D ms to serve)")
            .exclude("/metrics")
            .custom_level(|status| match status {
                // per-status levels
                StatusCode::METHOD_NOT_ALLOWED => Level::Warn,
                // general cases
                _ if status.is_server_error() => Level::Error,
                _ => Level::Info,
            })
            .custom_request_replace("request_id", |req| {
                req.extensions()
                    .get::<RequestId>()
                    .copied()
                    .map(|request_id| request_id.to_string())
                    .unwrap_or("-".to_string())
            })
    }
}
