#[cfg(feature = "actix-web")]
use {actix_contrib_logger::middleware::Logger, log::Level};

#[cfg(feature = "actix-web")]
pub struct CustomActixLogger;
#[cfg(feature = "actix-web")]
impl CustomActixLogger {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Logger {
        Logger::new("%a \"%r\" %s (took %D ms to serve)")
            .exclude("/metrics")
            .custom_level(|status| {
                if status.is_server_error() {
                    Level::Error
                } else {
                    Level::Debug
                }
            })
    }
}
