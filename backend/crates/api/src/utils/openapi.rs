use utoipa::openapi::security::{Http, HttpAuthScheme};
use utoipa::{
    Modify, OpenApi,
    openapi::{OpenApi as OpenApiStruct, security::SecurityScheme},
};
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
        )
    ),
    modifiers(&Security)
)]
pub struct Swagger;
impl Swagger {
    pub fn ui_service(api: OpenApiStruct) -> SwaggerUi {
        SwaggerUi::new("/swagger-ui/{_}*").url("/openapi.json", api)
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
    }
}
