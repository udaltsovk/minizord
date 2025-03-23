use crate::config;
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    error::OTelSdkResult,
    logs::{BatchLogProcessor, SdkLoggerProvider},
    trace::SdkTracerProvider,
};
use std::{str::FromStr, time::Duration};
use tracing::level_filters::LevelFilter;
use tracing_actix_web::{RootSpanBuilder, TracingLogger};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

pub struct LGTM {
    logger_provider: SdkLoggerProvider,
    tracer_provider: SdkTracerProvider,
}
impl LGTM {
    #[inline]
    const fn default_log_level() -> &'static str {
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        }
    }

    fn logger_provider() -> SdkLoggerProvider {
        SdkLoggerProvider::builder()
            .with_log_processor(
                BatchLogProcessor::builder(
                    LogExporter::builder()
                        .with_tonic()
                        .with_endpoint(config::OTEL_ENDPOINT.clone())
                        .with_timeout(Duration::from_secs(5))
                        .build()
                        .unwrap(),
                )
                .build(),
            )
            .build()
    }

    fn tracer_provider() -> SdkTracerProvider {
        SdkTracerProvider::builder()
            .with_batch_exporter(
                SpanExporter::builder()
                    .with_tonic()
                    .with_endpoint(config::OTEL_ENDPOINT.clone())
                    .with_timeout(Duration::from_secs(5))
                    .build()
                    .unwrap(),
            )
            .build()
    }

    pub fn init() -> Self {
        let logger_provider = Self::logger_provider();
        let tracer_provider = Self::tracer_provider();

        let filter_layer = EnvFilter::builder()
            .with_default_directive(
                LevelFilter::from_str(Self::default_log_level())
                    .unwrap()
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

        let fmt_layer = fmt::layer().compact();

        let log_layer = OpenTelemetryTracingBridge::new(&logger_provider);
        let tracer = tracer_provider.tracer(config::OTEL_SERVICE_NAME.clone());

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(log_layer)
            .with(OpenTelemetryLayer::new(tracer))
            .init();

        Self {
            logger_provider,
            tracer_provider,
        }
    }

    pub fn tracing_middleware() -> TracingLogger<impl RootSpanBuilder> {
        TracingLogger::default()
    }

    pub fn metrics_middleware() -> PrometheusMetrics {
        PrometheusMetricsBuilder::new(config::OTEL_SERVICE_NAME.as_str())
            .endpoint("/metrics")
            .mask_unmatched_patterns("UNKNOWN")
            .build()
            .expect("Failed to create prometheus metrics middleware")
    }

    pub fn shutdown(self) -> OTelSdkResult {
        self.tracer_provider.shutdown()?;

        self.logger_provider.shutdown()?;

        Ok(())
    }
}
