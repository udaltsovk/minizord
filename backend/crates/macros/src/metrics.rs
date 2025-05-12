#[macro_export]
macro_rules! metric_name {
    ($name:ident, $metric_name:literal) => {
        $crate::pastey::paste! {
            const [<$name _METRIC_NAME>]: &'static str = $metric_name;
        }
    };
}
