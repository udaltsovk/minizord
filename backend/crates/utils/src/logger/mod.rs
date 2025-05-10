#[cfg(feature = "actix-web")]
mod actix_web_specific;

#[cfg(feature = "actix-web")]
pub use actix_web_specific::CustomActixLogger;
