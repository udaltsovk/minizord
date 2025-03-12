#[macro_export]
macro_rules! to_snake_case {
    ($struct:ident) => {
        stringify!(macros::stringcase::snake_case($struct.to_string()))
    };
}
