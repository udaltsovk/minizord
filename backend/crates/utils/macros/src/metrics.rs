#[macro_export]
macro_rules! metric_name {
    ($name:ident, $metric_name:literal) => {
        $crate::paste! {
            const [<$name _METRIC_NAME>]: &'static str = $metric_name;
        }
    };
}
