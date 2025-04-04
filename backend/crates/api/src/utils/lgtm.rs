use std::time::Duration;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    error::OTelSdkResult,
    logs::{BatchLogProcessor, SdkLogger, SdkLoggerProvider},
    trace::{BatchSpanProcessor, SdkTracerProvider, Tracer},
};
use tracing::{Span, Subscriber, level_filters::LevelFilter};
use tracing_actix_web::{
    DefaultRootSpanBuilder, Level, RootSpanBuilder, TracingLogger,
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    EnvFilter,
    filter::Directive,
    fmt::{
        self, Layer,
        format::{Compact, DefaultFields, Format},
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

use crate::config;

#[inline]
fn parse_directive(directive: &'static str) -> Directive {
    directive.parse().expect("Failed to parse directive")
}

pub struct LGTM {
    logger_provider: SdkLoggerProvider,
    tracer_provider: SdkTracerProvider,
}
impl LGTM {
    #[inline]
    fn filter_layer() -> EnvFilter {
        EnvFilter::builder()
            .with_default_directive(
                if cfg!(debug_assertions) {
                    LevelFilter::DEBUG
                } else {
                    LevelFilter::INFO
                }
                .into(),
            )
            .from_env_lossy()
            .add_directive(parse_directive("tokio=off"))
            .add_directive(parse_directive("runtime=off"))
            .add_directive(parse_directive("hyper=off"))
            .add_directive(parse_directive("opentelemetry=off"))
            .add_directive(parse_directive("tonic=off"))
            .add_directive(parse_directive("h2=off"))
            .add_directive(parse_directive("tower=off"))
            .add_directive(parse_directive("reqwest=off"))
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
                    .expect("Failed to build exporter!"),
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
                    .expect("Failed to build exporter!"),
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
        TracingLogger::<CustomLevelRootSpanBuilder>::new()
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

pub struct CustomLevelRootSpanBuilder;

impl RootSpanBuilder for CustomLevelRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let level = if request.path() == "/metrics" {
            Level::TRACE
        } else {
            Level::INFO
        };
        tracing_actix_web::root_span!(level = level, request)
    }

    fn on_request_end<B: MessageBody>(
        span: Span,
        outcome: &Result<ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}
