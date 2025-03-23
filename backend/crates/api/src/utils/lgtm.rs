use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use std::time::Duration;
use tracing::level_filters::LevelFilter;
use tracing_actix_web::{RootSpanBuilder, TracingLogger};
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

use crate::config;

pub struct LGTM;
impl LGTM {
    pub fn init_logging() {
        let env_filter = EnvFilter::builder()
            .with_default_directive(
                if cfg!(debug_assertions) {
                    LevelFilter::DEBUG
                } else {
                    LevelFilter::INFO
                }
                .into(),
            )
            .from_env_lossy()
            .add_directive("tokio=off".parse().unwrap())
            .add_directive("runtime=off".parse().unwrap())
            .add_directive("hyper=off".parse().unwrap())
            .add_directive("opentelemetry=off".parse().unwrap())
            .add_directive("tonic=off".parse().unwrap())
            .add_directive("h2=off".parse().unwrap())
            .add_directive("tower=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap());

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(config::OTLP_ENDPOINT.clone())
            .with_timeout(Duration::from_secs(5));

        let telemetry = tracing_opentelemetry::layer().with_tracer(
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .install_batch(runtime::Tokio)
                .unwrap(),
        );

        let fmt_layer = fmt::layer().pretty();

        tracing::subscriber::set_global_default(
            Registry::default()
                .with(env_filter)
                .with(telemetry)
                .with(fmt_layer),
        )
        .expect("Could not set up global logger");
    }

    pub fn tracing() -> TracingLogger<impl RootSpanBuilder> {
        TracingLogger::default()
    }

    pub fn metrics() -> PrometheusMetrics {
        PrometheusMetricsBuilder::new("megazord")
            .endpoint("/metrics")
            .mask_unmatched_patterns("UNKNOWN")
            .build()
            .expect("Failed to create prometheus metrics middleware")
    }
}
