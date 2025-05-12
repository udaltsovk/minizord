use std::{
    borrow::Cow, net::SocketAddr, ops::Deref, str::FromStr, sync::Arc,
    time::Duration,
};

use macros::metric_name;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use metrics_process::Collector;
use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
use metrics_util::layers::Layer as _;
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{
    ExportConfig, LogExporter, Protocol, SpanExporter, WithExportConfig,
};
use opentelemetry_sdk::{
    Resource,
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
    actix_web_metrics::{
        ActixWebMetrics, ActixWebMetricsBuilder, ActixWebMetricsConfig,
        LabelsConfig,
    },
    actix_web_specific::CustomLevelRootSpanBuilder,
    tracing_actix_web::{RootSpanBuilder, TracingLogger},
};

#[cfg(feature = "actix-web")]
mod actix_web_specific;

#[inline]
fn parse_directive(directive: &'static str) -> Directive {
    directive.parse().expect("Failed to parse directive")
}

metric_name!(
    HTTP_REQUESTS_DURATION_SECONDS,
    "http_server_request_duration_seconds"
);

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: String,
    otel_service_name: Cow<'static, str>,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
    metrics_process_collector: Arc<Collector>,
}
impl LGTM {
    pub fn get_logger_provider(&self) -> SdkLoggerProvider {
        self.logger_provider
            .clone()
            .expect("Called `LGTM::get_logger_provider` too early")
            .deref()
            .clone()
    }

    pub fn get_tracer_provider(&self) -> SdkTracerProvider {
        self.tracer_provider
            .clone()
            .expect("Called `LGTM::get_tracer_provider` too early")
            .deref()
            .clone()
    }

    #[inline]
    fn export_config(&self) -> ExportConfig {
        ExportConfig {
            protocol: Protocol::Grpc,
            endpoint: Some(self.otel_endpoint.clone()),
            timeout: Some(Duration::from_secs(30)),
        }
    }

    #[inline]
    fn configure_logger_provider(mut self) -> Self {
        let logger_provider = SdkLoggerProvider::builder()
            .with_resource(self.resource.clone())
            .with_log_processor(
                BatchLogProcessor::builder(
                    LogExporter::builder()
                        .with_tonic()
                        .with_export_config(self.export_config())
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
            .with_resource(self.resource.clone())
            .with_span_processor(
                BatchSpanProcessor::builder(
                    SpanExporter::builder()
                        .with_tonic()
                        .with_export_config(self.export_config())
                        .build()
                        .expect("Failed to build exporter!"),
                )
                .build(),
            )
            .build();
        global::set_tracer_provider(tracer_provider.clone());
        self.tracer_provider = Some(Arc::new(tracer_provider));
        self
    }

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
    fn log_layer(
        &self,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(&self.get_logger_provider())
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
        otel_endpoint: &'static str,
        otel_service_name: &'static str,
        prometheus_address: &'static str,
    ) -> Self {
        let lgtm = Self {
            otel_endpoint: otel_endpoint.into(),
            otel_service_name: otel_service_name.into(),
            resource: Resource::builder()
                .with_service_name(otel_service_name)
                .build(),
            logger_provider: None,
            tracer_provider: None,
            metrics_process_collector: Arc::new(Collector::default()),
        }
        .configure_logger_provider()
        .configure_tracer_provider();

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .with(MetricsLayer::new())
            .init();

        let (prometheus_recorder, serve_prometheus) = PrometheusBuilder::new()
            .with_http_listener(
                SocketAddr::from_str(prometheus_address)
                    .expect("a valid address"),
            )
            .set_buckets_for_metric(
                Matcher::Full(
                    HTTP_REQUESTS_DURATION_SECONDS_METRIC_NAME.to_string(),
                ),
                &[
                    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
                    10.0,
                ],
            )
            .expect("values to be not empty")
            .build()
            .expect("Failed to build Prometheus");

        tokio::spawn(serve_prometheus);

        metrics::set_global_recorder(
            TracingContextLayer::all().layer(prometheus_recorder),
        )
        .expect("Failed to set up global metrics recorder");

        lgtm.metrics_process_collector.describe();

        tokio::spawn(
            tokio_metrics::RuntimeMetricsReporterBuilder::default()
                .describe_and_run(),
        );

        let collector = lgtm.metrics_process_collector.clone();
        tokio::spawn(async move {
            loop {
                collector.collect();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        lgtm
    }

    #[cfg(feature = "actix-web")]
    pub fn tracing_middleware() -> TracingLogger<impl RootSpanBuilder> {
        TracingLogger::<CustomLevelRootSpanBuilder>::new()
    }

    #[cfg(feature = "actix-web")]
    pub fn metrics_middleware(&self) -> ActixWebMetrics {
        ActixWebMetricsBuilder::new()
            .mask_unmatched_patterns("UNKNOWN")
            .metrics_config(
                ActixWebMetricsConfig::default()
                    .http_requests_duration_seconds_name(
                        HTTP_REQUESTS_DURATION_SECONDS_METRIC_NAME,
                    )
                    .labels(
                        LabelsConfig::default()
                            .status("http_response_status_code")
                            .endpoint("http_route")
                            .version("network_protocol_version"),
                    ),
            )
            .build()
            .expect("Failed to create prometheus metrics middleware")
    }

    pub fn shutdown(&self) -> OTelSdkResult {
        tracing::info!("Shutting down LGTM stuff");

        self.get_tracer_provider().shutdown()?;
        self.get_logger_provider().shutdown()?;

        Ok(())
    }
}
