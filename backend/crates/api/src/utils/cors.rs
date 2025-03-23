use actix_cors::Cors;

#[tracing::instrument(level = "trace")]
pub fn default_cors() -> Cors {
    Cors::permissive()
}
