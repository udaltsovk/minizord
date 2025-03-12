#[macro_export]
macro_rules! service {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty
            );*
        }
    ) => {
        macros::paste::paste! {
            type [<$name ServiceError>] = crate::common::ServiceError;

            #[macros::async_trait::async_trait]
            pub trait [<$name Service>] {
                $(
                    $(#[$fn_meta])*
                    async fn $method $sig-> Result<$res, [<$name ServiceError>]>;
                )*
            }

        }
    };
}
