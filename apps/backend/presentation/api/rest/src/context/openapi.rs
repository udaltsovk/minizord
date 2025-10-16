use utoipa::{
    Modify, OpenApi as OpenApiDerive,
    openapi::{
        OpenApi,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
};

use crate::routes::{sessions::SESSIONS_TAG, users::USERS_TAG};

#[derive(OpenApiDerive)]
#[openapi(
    info(
        title = "Minizord API",
        description = "OpenAPI for the Minizord platform",
        contact(
            name = "GitHub repo",
            url = "https://github.com/udaltsovk/minizord"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Local instance"),
    ),
    tags(
        (name = USERS_TAG, description = ""),
        (name = SESSIONS_TAG, description = ""),
    ),
    modifiers(
        &SecurityModifier,
    ),
)]
pub struct ApiDoc;

struct SecurityModifier;
impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("Failed do get mutable components");

        let scheme = SecurityScheme::Http(
            Http::builder()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        components.add_security_scheme("admin", scheme.clone());
        components.add_security_scheme("organizer", scheme.clone());
        components.add_security_scheme("mentor", scheme.clone());
        components.add_security_scheme("participant", scheme.clone());
    }
}
