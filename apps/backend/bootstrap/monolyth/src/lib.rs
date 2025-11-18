use std::{net::IpAddr, str::FromStr as _};

use env_vars_config::env_vars_config;

pub mod bootstrappers;
mod modules;

pub use modules::Modules;

env_vars_config! {
    SERVER_ADDRESS: IpAddr = IpAddr::from_str("0.0.0.0").expect("a valid IP address"),
    SERVER_PORT: u16 = 8080u16,
    SURREAL_ADDRESS: String = "localhost:8001",
    SURREAL_NAMESPACE: String = "minizord",
    SURREAL_DATABASE: String = "monolyth",
    SURREAL_USERNAME: String = "root",
    SURREAL_PASSWORD: String = "root",
    SURREAL_MAX_POOL_SIZE: u64 = 16u64,
    OTEL_ENDPOINT: String = "http://localhost:4317",
    OTEL_SERVICE_NAMESPACE: String = "minizord",
    OTEL_SERVICE_NAME: String = "monolyth",
    JWT_SECRET: String = "changeme",
    BASE_API_URL: String = "http://localhost:8080",
}
