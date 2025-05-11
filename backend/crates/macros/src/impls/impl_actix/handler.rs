#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! handler {
    (
        $(#[$meta:meta])*
        $name:ident
            Err: $err_ty:ty
            $(, Impl: $impl_tt:tt)?
        {
            $(#[$routes_meta:meta])*
            fn routes $routes_sig:tt
                $routes_body:block

            $(
                $(#[$fn_meta:meta])*
                async fn $method:ident $sig:tt -> $res:ty
            );*
            $(;)?
        }
    ) => {
        $crate::pastey::paste! {
            type [<$name HandlerResult>]<T> = Result<T, $err_ty>;

            pub trait [<$name Handler>] {
                $(
                    $(#[$fn_meta])*
                    fn $method () -> impl actix_web::dev::HttpServiceFactory + utoipa_actix_web::OpenApiFactory + 'static;
                )*

                fn routes $routes_sig -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
                where
                    Self: Sized + Clone + 'static
                {
                    $routes_body
                }
            }

            #[$crate::async_trait::async_trait]
            trait [<$name HandlerHelper>] {
                $(
                    async fn $method $sig -> [<$name HandlerResult>]<$res>;
                )*
            }

        }
    };
}
