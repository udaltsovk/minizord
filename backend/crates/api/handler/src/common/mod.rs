mod error;
pub mod middleware;

pub use error::{
    ApiError, HandlerError, auth::AuthenticationError,
    validation::ValidationError,
};
pub use utoipa::path as openapi;
