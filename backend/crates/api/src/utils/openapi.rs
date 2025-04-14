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
        title = "Minizord API",
        description = "OpenAPI for the Minizord platform",
        contact(
            name = "GitHub repo",
            url = "https://github.com/udaltsovk/minizord"
        )
    ),
    servers(
        (
            url = "https://minizord-api.udaltsovk.ru/",
            description = "Hosted instance"
        ),
        (
            url = "http://localhost:8080/",
            description = "Local server"
        )
    ),
    tags(
        (
            name = "Info",
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
        (
            name = "Reviews",
            description = "Review-related endpoints"
        ),
    ),
    nest(
        (
            path = "/",
            api = handler::info::implementation::OpenApi,
            tags = ["Info"]
        ),
        (
            path = "/users",
            api = handler::user::implementation::OpenApi,
            tags = ["Users"]
        ),
        (
            path = "/profiles",
            api = handler::profile::implementation::OpenApi,
            tags = ["Profiles"]
        ),
        (
            path = "/reviews",
            api = handler::review::implementation::OpenApi,
            tags = ["Reviews"]
        ),
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

    pub fn json_string() -> String {
        OpenApi::openapi()
            .to_pretty_json()
            .expect("Failed to build pretty-printed OpenApi JSON")
    }
}

struct Security;
impl Modify for Security {
    fn modify(&self, openapi: &mut OpenApiStruct) {
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

        components.add_security_scheme("organizator", scheme.clone());
        components.add_security_scheme("mentor", scheme.clone());
        components.add_security_scheme("participant", scheme.clone());
    }
}
