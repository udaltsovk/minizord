use std::str::FromStr;

use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    web::Data,
};
use dto::user::UserRole;
use macros::auth_middlewares;
use service::user::UserServiceDependency;
use ulid::Ulid;
use utils::{
    auth::{jsonwebtoken, jwt},
    chrono::Utc,
};

use crate::common::{AuthenticationError, HandlerError};

auth_middlewares! {
    access_levels: [Participant, Mentor, Organizator],
}

#[inline]
pub async fn auth_middleware(
    jwt_secret: Data<String>,
    user_service: Data<UserServiceDependency>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = extract_auth_from_authorization_header(&req)?;

    let claims = match jwt::parse(&token, &jwt_secret) {
        None => Err(AuthenticationError::InvalidCredentials)?,
        Some(claims) => claims,
    };

    if claims.iat
        >= usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX)
    {
        Err(AuthenticationError::InvalidCredentials)?;
    }

    let id = Ulid::from_string(&claims.sub)
        .map_err(|_| AuthenticationError::InvalidCredentials)?;

    let user = match user_service
        .find_by_id(id)
        .await
        .map_err(HandlerError::from)?
    {
        None => Err(AuthenticationError::InvalidCredentials)?,
        Some(user) => user,
    };

    match jsonwebtoken::decode_header(&token).map(|h| h.kid) {
        Ok(Some(token_type))
            if UserRole::from_str(&token_type) == Ok(user.role) => {},
        _ => Err(AuthenticationError::InvalidCredentials)?,
    }

    req.extensions_mut().insert(user);

    next.call(req).await
}

#[inline]
#[tracing::instrument(skip_all, level = "trace")]
pub fn extract_auth_from_authorization_header(
    req: &ServiceRequest,
) -> Result<String, AuthenticationError> {
    if let Some(token) = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthenticationError::NoAuthorizationHeader)?
        .to_str()
        .map_err(|_| AuthenticationError::InvalidCredentials)?
        .strip_prefix("Bearer ")
    {
        Ok(token.to_string())
    } else {
        Err(AuthenticationError::InvalidAuthMethod)
    }
}
