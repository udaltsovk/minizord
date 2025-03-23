use actix_contrib_logger::middleware::Logger;
use log::Level;

pub struct CustomLogger;

impl CustomLogger {
    pub fn new() -> Logger {
        Logger::new("%a \"%r\" %s (took %D ms to serve)").custom_level(|status| {
            if status.is_server_error() {
                Level::Error
            } else {
                Level::Debug
            }
        })
    }
}
