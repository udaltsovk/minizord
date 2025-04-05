use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use tracing::{Level, Span};
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};

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
