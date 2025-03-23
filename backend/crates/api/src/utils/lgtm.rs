use std::{str::FromStr, time::Duration};

use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    error::OTelSdkResult,
    logs::{BatchLogProcessor, SdkLogger, SdkLoggerProvider},
    trace::{BatchSpanProcessor, SdkTracerProvider, Tracer},
};
use tracing::{Subscriber, level_filters::LevelFilter};
use tracing_actix_web::{RootSpanBuilder, TracingLogger};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    EnvFilter,
    fmt::{
        self, Layer,
        format::{Compact, DefaultFields, Format},
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

use crate::config;

pub struct LGTM {
    logger_provider: SdkLoggerProvider,
    tracer_provider: SdkTracerProvider,
}
impl LGTM {
    #[inline]
    fn filter_layer() -> EnvFilter {
        EnvFilter::builder()
            .with_default_directive(
                LevelFilter::from_str(if cfg!(debug_assertions) {
                    "debug"
                } else {
                    "info"
                })
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
            .add_directive("reqwest=off".parse().unwrap())
    }

    #[inline]
    fn fmt_layer<S>() -> Layer<S, DefaultFields, Format<Compact>> {
        fmt::layer().compact()
    }

    #[inline]
    fn configure_exporter<T: WithExportConfig>(exporter: T) -> T {
        exporter
            .with_endpoint(config::OTEL_ENDPOINT.clone())
            .with_timeout(Duration::from_secs(5))
    }

    #[inline]
    fn logger_provider() -> SdkLoggerProvider {
        SdkLoggerProvider::builder()
            .with_log_processor(
                BatchLogProcessor::builder(
                    Self::configure_exporter(
                        LogExporter::builder().with_tonic(),
                    )
                    .build()
                    .unwrap(),
                )
                .build(),
            )
            .build()
    }

    #[inline]
    fn tracer_provider() -> SdkTracerProvider {
        SdkTracerProvider::builder()
            .with_span_processor(
                BatchSpanProcessor::builder(
                    Self::configure_exporter(
                        SpanExporter::builder().with_tonic(),
                    )
                    .build()
                    .unwrap(),
                )
                .build(),
            )
            .build()
    }

    #[inline]
    fn log_layer(
        provider: &SdkLoggerProvider,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(provider)
    }

    #[inline]
    fn trace_layer<S: Subscriber + for<'span> LookupSpan<'span>>(
        provider: &SdkTracerProvider,
    ) -> OpenTelemetryLayer<S, Tracer> {
        OpenTelemetryLayer::new(
            provider.tracer(config::OTEL_SERVICE_NAME.clone()),
        )
    }

    pub fn init() -> Self {
        let logger_provider = Self::logger_provider();
        let tracer_provider = Self::tracer_provider();

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(Self::log_layer(&logger_provider))
            .with(Self::trace_layer(&tracer_provider))
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

    pub fn shutdown(&self) -> OTelSdkResult {
        log::info!("Shutting down LGTM stack");

        self.tracer_provider.shutdown()?;
        self.logger_provider.shutdown()?;

        Ok(())
    }
}
impl Drop for LGTM {
    fn drop(&mut self) {
        self.shutdown().expect("Failed to shut down LGTM stack");
    }
}
