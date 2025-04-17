use std::{borrow::Cow, sync::Arc, time::Duration};

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    error::OTelSdkResult,
    logs::{BatchLogProcessor, SdkLogger, SdkLoggerProvider},
    trace::{BatchSpanProcessor, SdkTracerProvider, Tracer},
};
use tracing::{Subscriber, level_filters::LevelFilter};
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
#[cfg(feature = "actix-web")]
use {
    actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder},
    actix_web_specific::CustomLevelRootSpanBuilder,
    tracing_actix_web::{RootSpanBuilder, TracingLogger},
};

#[cfg(feature = "actix-web")]
mod actix_web_specific;

#[inline]
fn parse_directive(directive: &'static str) -> Directive {
    directive.parse().expect("Failed to parse directive")
}

#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: String,
    otel_service_name: Cow<'static, str>,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
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
            .add_directive(parse_directive("aws=off"))
            .add_directive(parse_directive("rustls=off"))
    }

    #[inline]
    fn fmt_layer<S>() -> Layer<S, DefaultFields, Format<Compact>> {
        fmt::layer().compact()
    }

    #[inline]
    fn configure_exporter<T: WithExportConfig>(&self, exporter: T) -> T {
        exporter
            .with_endpoint(&self.otel_endpoint)
            .with_timeout(Duration::from_secs(5))
    }

    #[inline]
    fn configure_logger_provider(mut self) -> Self {
        let logger_provider = SdkLoggerProvider::builder()
            .with_log_processor(
                BatchLogProcessor::builder(
                    self.configure_exporter(
                        LogExporter::builder().with_tonic(),
                    )
                    .build()
                    .expect("Failed to build exporter!"),
                )
                .build(),
            )
            .build();
        self.logger_provider = Some(Arc::new(logger_provider));
        self
    }

    #[inline]
    fn configure_tracer_provider(mut self) -> Self {
        let tracer_provider = SdkTracerProvider::builder()
            .with_span_processor(
                BatchSpanProcessor::builder(
                    self.configure_exporter(
                        SpanExporter::builder().with_tonic(),
                    )
                    .build()
                    .expect("Failed to build exporter!"),
                )
                .build(),
            )
            .build();
        self.tracer_provider = Some(Arc::new(tracer_provider));
        self
    }

    #[inline]
    fn log_layer(
        &self,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(
            &self
                .logger_provider
                .clone()
                .expect("Called `LGTM::trace_layer` too early"),
        )
    }

    #[inline]
    fn trace_layer<S: Subscriber + for<'span> LookupSpan<'span>>(
        &self,
    ) -> OpenTelemetryLayer<S, Tracer> {
        OpenTelemetryLayer::new(
            self.tracer_provider
                .clone()
                .expect("Called `LGTM::trace_layer` too early")
                .tracer(self.otel_service_name.clone()),
        )
    }

    pub fn init(
        otel_endpoint: impl Into<String>,
        otel_service_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        let lgtm = Self {
            otel_endpoint: otel_endpoint.into(),
            otel_service_name: otel_service_name.into(),
            logger_provider: None,
            tracer_provider: None,
        }
        .configure_logger_provider()
        .configure_tracer_provider();

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .init();
        lgtm
    }

    #[cfg(feature = "actix-web")]
    pub fn tracing_middleware() -> TracingLogger<impl RootSpanBuilder> {
        TracingLogger::<CustomLevelRootSpanBuilder>::new()
    }

    #[cfg(feature = "actix-web")]
    pub fn metrics_middleware(&self) -> PrometheusMetrics {
        PrometheusMetricsBuilder::new(&self.otel_service_name)
            .endpoint("/metrics")
            .mask_unmatched_patterns("UNKNOWN")
            .build()
            .expect("Failed to create prometheus metrics middleware")
    }

    pub fn shutdown(&self) -> OTelSdkResult {
        tracing::info!("Shutting down LGTM stuff");

        self.tracer_provider
            .clone()
            .expect("Called `LGTM::shutdown` too early")
            .shutdown()?;
        self.logger_provider
            .clone()
            .expect("Called `LGTM::shutdown` too early")
            .shutdown()?;

        Ok(())
    }
}
