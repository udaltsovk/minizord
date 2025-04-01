#[macro_export]
macro_rules! auth_middlewares {
    (
        access_levels: [$($access_level:ident),*],
        $(,)?
    ) => {
        macros::paste::paste! {
            #[derive(PartialEq, strum_macros::Display, Debug)]
            pub enum TokenType {
                $($access_level),*
            }

            $(
                #[tracing::instrument(skip_all)]
                pub async fn [<$access_level:snake _auth_middleware>](
                    user: actix_web::web::ReqData<dto::user::User>,
                    req: actix_web::dev::ServiceRequest,
                    next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
                ) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
                    if dto::user::UserRole::from_str(stringify!([<$access_level:snake>])) == Ok(user.role) {
                        Err(AuthenticationError::MissingPermissions)?
                    }

                    next.call(req).await
                }
            )*
        }
    };
}
