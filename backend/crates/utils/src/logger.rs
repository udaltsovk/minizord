#[cfg(feature = "actix-web")]
use {actix_contrib_logger::middleware::Logger, tracing::log::Level};

#[cfg(feature = "actix-web")]
pub struct CustomActixLogger;
#[cfg(feature = "actix-web")]
impl CustomActixLogger {
    #[allow(
        clippy::new_ret_no_self,
        clippy::match_single_binding,
        clippy::if_same_then_else
    )]
    pub fn new() -> Logger {
        Logger::new("%{r}a \"%r\" %s (took %D ms to serve)")
            .exclude("/metrics")
            .custom_level(|status| {
                if status.is_server_error() {
                    match status {
                        _ => Level::Error,
                    }
                } else if status.is_client_error() {
                    match status {
                        _ => Level::Info,
                    }
                } else if status.is_redirection() {
                    match status {
                        _ => Level::Info,
                    }
                } else if status.is_success() {
                    match status {
                        _ => Level::Info,
                    }
                } else if status.is_informational() {
                    match status {
                        _ => Level::Info,
                    }
                } else {
                    Level::Info
                }
            })
    }
}
