use actix_cors::Cors;

#[tracing::instrument]
pub fn default_cors() -> Cors {
    Cors::permissive()
}
