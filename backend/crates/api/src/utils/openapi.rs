use handler;
use utoipa::{
    Modify, OpenApi as OpenApiTrait,
    openapi::{
        OpenApi as OpenApiStruct, Server,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
};
#[cfg(feature = "scalar")]
use utoipa_scalar::{Scalar, Servable};
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

use crate::config;

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
    modifiers(
        &ServerModifier,
        &SecurityModifier,
    ),
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

struct ServerModifier;
impl Modify for ServerModifier {
    fn modify(&self, openapi: &mut OpenApiStruct) {
        openapi.servers = Some(vec![
            Server::builder()
                .url(config::BASE_API_URL.to_string())
                .description(Some(
                    if config::DEPLOY_DOMAIN.as_str() == "localhost" {
                        "Local server"
                    } else {
                        "Hosted instance"
                    },
                ))
                .build(),
        ]);
    }
}

struct SecurityModifier;
impl Modify for SecurityModifier {
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

        components.add_security_scheme("organizer", scheme.clone());
        components.add_security_scheme("mentor", scheme.clone());
        components.add_security_scheme("participant", scheme.clone());
    }
}
