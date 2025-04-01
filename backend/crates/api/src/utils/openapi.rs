use handler;
use utoipa::{
    Modify, OpenApi as OpenApiTrait,
    openapi::{
        OpenApi as OpenApiStruct,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
};
#[cfg(feature = "scalar")]
use utoipa_scalar::{Scalar, Servable};
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApiTrait, Debug)]
#[openapi(
    info(
        title = "Megazord API",
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
            name = "Users",
            description = "User-related endpoints"
        ),
        (
            name = "Profiles",
            description = "Profile-related endpoints"
        ),
    ),
    nest(
        (path = "/health", api = handler::health::OpenApi, tags = ["Health"]),
        (path = "/users", api = handler::user::OpenApi, tags = ["Users"]),
        (path = "/profiles", api = handler::profile::OpenApi, tags = ["Profiles"]),
    ),
    modifiers(&Security),
)]
pub struct OpenApi;
impl OpenApi {
    #[cfg(feature = "scalar")]
    pub fn ui_service(api: OpenApiStruct) -> Scalar<OpenApiStruct> {
        Scalar::with_url("/openapi", api)
    }

    #[cfg(feature = "swagger")]
    pub fn ui_service(api: OpenApiStruct) -> SwaggerUi {
        SwaggerUi::new("/openapi/{_}*").url("/openapi.json", api)
    }

    pub fn as_json() -> String {
        OpenApi::openapi().to_pretty_json().unwrap()
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
