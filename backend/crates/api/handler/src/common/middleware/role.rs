use std::task::{Context, Poll};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use dto::user::{User, UserRole};
use futures_util::future::{LocalBoxFuture, Ready, ready};

use crate::common::AuthenticationError;

pub struct UserRoleFilterMiddleware {
    allowed_roles: Vec<UserRole>,
}

impl UserRoleFilterMiddleware {
    pub fn new(allowed_roles: Vec<UserRole>) -> Self {
        Self {
            allowed_roles,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for UserRoleFilterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = UserRoleFilterMiddlewareService<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserRoleFilterMiddlewareService {
            service,
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct UserRoleFilterMiddlewareService<S> {
    service: S,
    allowed_roles: Vec<UserRole>,
}

impl<S, B> Service<ServiceRequest> for UserRoleFilterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<B>;

    fn poll_ready(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    #[tracing::instrument(name = "role_filter", skip_all, level = "info")]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let allowed = req
            .extensions()
            .get::<User>()
            .map(|u| self.allowed_roles.contains(&u.role))
            .unwrap_or(false);

        if !allowed {
            return Box::pin(async {
                Err(AuthenticationError::MissingPermissions.into())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
