#[macro_export]
macro_rules! surql_query {
    ($path:literal) => {
        include_str!(concat!("../../db/surreal/queries/", $path, ".surql"))
    };
}
