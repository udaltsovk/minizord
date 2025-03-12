#[macro_export]
macro_rules! handler {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty
            );*
            $(;)?
        }
    ) => {
        macros::paste::paste! {
            // Generate handler trait
            #[macros::async_trait::async_trait]
            pub trait [<$name Handler>] {
                $(
                    $(#[$fn_meta])*
                    async fn $method $sig-> Result<$res, crate::common::HandlerError>;
                )*

                fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig);
            }

        }
    };
}
