#![allow(clippy::unwrap_used)]

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    ///
    pub static ref RE_USERNAME: Regex =
        Regex::new(r#"^[a-zA-Z0-9._-]{3,20}$"#)
            .unwrap();
    ///
    pub static ref RE_SENTENCE: Regex =
        Regex::new(r#"a-zA-Zа-яёА-ЯЁ0-9._=\ "#)
            .unwrap();
    ///
    pub static ref RE_NAME: Regex =
        Regex::new(r#"^[А-ЯЁ]{1}[а-яё]{1,23}$"#)
            .unwrap();
    ///
    pub static ref RE_TELEGRAM_USERNAME: Regex =
        Regex::new(r#"^[A-Za-z\d_]{5,32}$"#)
            .unwrap();
}
