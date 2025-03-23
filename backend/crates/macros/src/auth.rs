#[macro_export]
macro_rules! auth_middlewares {
    (
        access_levels: [$($access_level:ident),*],
        entities: [$($entity:tt),*]
        $(,)?
    ) => {
        macros::paste::paste! {
            #[derive(Clone, Debug)]
            pub enum AuthEntity {
                $($entity($entity)),*
            }
            $(
                impl TryInto<$entity> for AuthEntity {
                    type Error = AuthenticationError;
                    #[tracing::instrument(skip_all, level = "trace")]
                    fn try_into(self) -> Result<$entity, Self::Error> {
                        match self {
                            AuthEntity::$entity(entity) => Ok(entity),
                            #[allow(unreachable_patterns)]
                            _ => Err(AuthenticationError::MissingPermissions)?,
                        }
                    }
                }
            )*

            #[derive(PartialEq, Debug)]
            pub enum TokenType {
                $($access_level),*
            }

            $(
                // TODO: Make services generate automatically
                // I can't find a way to make composed meta-variable arrays with different sizes from non-copmposed ones
                #[tracing::instrument(skip_all)]
                pub async fn [<$access_level:snake _auth_middleware>](
                    jwt_secret: actix_web::web::Data<String>,
                    participant_service: actix_web::web::Data<service::participant::ParticipantServiceDependency>, // TODO: here
                    mentor_service: actix_web::web::Data<service::mentor::MentorServiceDependency>, // TODO: here
                    organizator_service: actix_web::web::Data<service::organizator::OrganizatorServiceDependency>, // TODO: here
                    req: actix_web::dev::ServiceRequest,
                    next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
                ) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
                    auth_middleware(
                        jwt_secret,
                        participant_service, // TODO: here
                        mentor_service, // TODO: here
                        organizator_service, // TODO: and here
                        TokenType::$access_level,
                        req,
                        next
                    ).await
                }
            )*

            #[tracing::instrument(skip_all, level = "debug")]
            pub async fn auth_middleware(
                jwt_secret: actix_web::web::Data<String>,
                $(
                    [<$entity:snake _service>]: actix_web::web::Data<service::[<$entity:snake>]::[<$entity ServiceDependency>]>,
                )*
                access_level: TokenType,
                req: actix_web::dev::ServiceRequest,
                next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
            ) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
                let token = extract_auth_from_authorization_header(&req)?;

                let claims = match utils::auth::jwt::parse(&token, &jwt_secret) {
                    None => Err(AuthenticationError::InvalidCredentials)?,
                    Some(claims) => claims,
                };

                if claims.iat >= utils::chrono::Utc::now().timestamp() as usize {
                    Err(AuthenticationError::InvalidCredentials)?;
                }

                let id = ulid::Ulid::from_string(&claims.sub)
                    .map_err(|_| AuthenticationError::InvalidCredentials)?;

                use actix_web::HttpMessage;

                let token_type = match utils::auth::jsonwebtoken::decode_header(&token)
                    .map(|h| h.kid.map(|k| k.clone()))
                {
                    $(
                        Ok(Some(token_type)) if &token_type == stringify!([<$entity:snake>]) => {
                            let [<$entity:snake>] = match [<$entity:snake _service>]
                                .find_by_id(id)
                                .await
                                .map_err(HandlerError::from)?
                            {
                                None => Err(AuthenticationError::InvalidCredentials)?,
                                Some([<$entity:snake>]) => [<$entity:snake>],
                            };

                            req.extensions_mut()
                                .insert(AuthEntity::$entity([<$entity:snake>]));

                            TokenType::$entity
                        }
                    ),*
                    _ => Err(AuthenticationError::InvalidCredentials)?,
                };

                match access_level {
                    TokenType::Any => (),
                    level => {
                        if level != token_type {
                            Err(AuthenticationError::MissingPermissions)?;
                        }
                    }
                }

                next.call(req).await
            }
        }
    };
}
