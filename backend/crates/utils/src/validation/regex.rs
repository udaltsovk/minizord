use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RE_USERNAME: Regex = Regex::new(r#"^[a-zA-Z0-9._-]*$"#).unwrap();
}
