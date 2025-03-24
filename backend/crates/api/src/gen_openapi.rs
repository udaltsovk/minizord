use std::fs;

use api::utils::openapi::OpenApi;

fn main() -> std::io::Result<()> {
    fs::write("./assets/openapi.json", OpenApi::as_json())
}
