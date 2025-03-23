use actix_web::{
    dev::ServiceRequest,
    http::header::{AUTHORIZATION, HeaderValue},
};
use dto::{mentor::Mentor, organizator::Organizator, participant::Participant};
use macros::auth_middlewares;

use crate::common::{AuthenticationError, HandlerError};

auth_middlewares! {
    access_levels: [Any, Participant, Mentor, Organizator],
    entities: [Participant, Mentor, Organizator]
}

#[inline]
#[tracing::instrument(skip_all, level = "trace")]
pub fn extract_auth_from_authorization_header(
    req: &ServiceRequest,
) -> Result<String, AuthenticationError> {
    let headers = req.headers();
    let token_val: Option<&HeaderValue> = headers.get(AUTHORIZATION);
    let token_val = token_val
        .ok_or(AuthenticationError::NoAuthorizationHeader)?
        .to_str()
        .map_err(|_| AuthenticationError::InvalidCredentials)?;

    if let Some(token) = token_val.strip_prefix("Bearer ") {
        Ok(token.to_string())
    } else {
        Err(AuthenticationError::InvalidAuthMethod)
    }
}
