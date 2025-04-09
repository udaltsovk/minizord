#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! service {
    (
        $(#[$meta:meta])*
        $name:ident
            Err: $err_ty:ty
        {
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        }
    ) => {
        $crate::pastey::paste! {
            type [<$name ServiceResult>]<T> = Result<T, $err_ty>;

            #[$crate::async_trait::async_trait]
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
