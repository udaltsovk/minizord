use std::time::Duration;

use lib::{
    bootstrap::{bootstrap, configure_jemalloc},
    infrastructure::instrumentation::opentelemetry::LGTM,
    presentation::api::rest::startup::RestApi,
};
use minizord_monolyth::{Modules, config};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    config::init();

    // bootstrap!(minizord_monolyth, [RestApi], Modules::init()).await

    LGTM::new(&config::OTEL_SERVICE_NAMESPACE, &config::OTEL_SERVICE_NAME)
        .with_otel_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(minizord_monolyth, [RestApi], Modules::init()))
        .await
}
