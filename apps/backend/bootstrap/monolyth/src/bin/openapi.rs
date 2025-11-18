use lib::bootstrap::openapi::{OpenAPISaver as _, OpenAPISaverResult};
use minizord_monolyth::Modules;
use presentation::api::rest::routes;

fn main() -> OpenAPISaverResult {
    routes::router::<Modules>()
        .into_openapi()
        .save_as("minizord")
}
