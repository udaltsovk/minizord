#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! service {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        }
    ) => {
        macros::paste::paste! {
            type [<$name ServiceResult>]<T> = Result<T, crate::common::ServiceError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Service>] {
                $(
                    $(#[$fn_meta])*
                    async fn $method $sig-> [<$name ServiceResult>]<$res>;
                )*
            }

            pub type [<$name ServiceDependency>] = std::sync::Arc<dyn [<$name Service>] + Send + Sync>;
        }
    };
}
