use actix_cors::Cors;

pub fn default_cors() -> Cors {
    Cors::permissive()
}
