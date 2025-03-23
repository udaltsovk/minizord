use utoipa::openapi::security::{Http, HttpAuthScheme};
use utoipa::{
    Modify, OpenApi,
    openapi::{OpenApi as OpenApiStruct, security::SecurityScheme},
};
#[cfg(feature = "scalar")]
use utoipa_scalar::{Scalar, Servable};
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi, Debug)]
#[openapi(
    info(
        title = "Megazord API",
        description = "OpenAPI for the Megazord platform",
        version = "0.1.0",
        contact(
            name = "GitHub repo",
            url = "https://github.com/udaltsovk/megazord"
        )
    ),
    servers(
        (
            url = "http://localhost:8080/",
            description = "Local server"
        )
    ),
    tags(
        (
            name = "Health",
            description = "Endpoints for monitoring"
        ),
        (
            name = "Organizators",
            description = "Organizator-related endpoints"
        ),
        (
            name = "Mentors",
            description = "Mentor-related endpoints"
        ),
        (
            name = "Participants",
            description = "Participant-related endpoints"
        ),
    ),
    modifiers(&Security)
)]
pub struct OpenApiVisualiser;
impl OpenApiVisualiser {
    #[cfg(feature = "scalar")]
    pub fn service(api: OpenApiStruct) -> Scalar<OpenApiStruct> {
        Scalar::with_url("/openapi", api)
    }
    #[cfg(feature = "swagger")]
    pub fn service(api: OpenApiStruct) -> SwaggerUi {
        SwaggerUi::new("/openapi/{_}*").url("/openapi.json", api)
    }
}

struct Security;
impl Modify for Security {
    fn modify(&self, openapi: &mut OpenApiStruct) {
        let components = openapi.components.as_mut().unwrap();

        let scheme = SecurityScheme::Http(
            Http::builder()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        components.add_security_scheme("organizator", scheme.clone());
        components.add_security_scheme("mentor", scheme.clone());
        components.add_security_scheme("participant", scheme.clone());
    }
}
