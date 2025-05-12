#[macro_export]
macro_rules! implementation {
    (
        $(#[$impl_meta:meta])*
        $trait_name:ident $({
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $ty:ty
            ),* $(,)?
        })? as $impl_name:ident {
            $(
                $(#[$method_meta:meta])*
                async fn $method:ident $sig:tt $(-> $res:ty)?
                    $body:block
            )*
        }
    ) => {
        $crate::pastey::paste! {
            $(#[$impl_meta])*
            pub struct $impl_name {
                $($(
                    $(#[$field_meta])*
                    $field_vis $field: $ty,
                )*)?
            }
            impl $impl_name {
                #[tracing::instrument(skip_all, level = "trace")]
                pub fn new($($($field: $ty),*)?) -> std::sync::Arc<Self> {
                    std::sync::Arc::new(Self {
                        $(
                            $($field),*
                        )?
                    })
                }
            }

            #[$crate::async_trait::async_trait]
            impl $trait_name for $impl_name {
                $(
                    $(#[$method_meta])*
                    async fn $method $sig $(-> [<$trait_name Result>]<$res>)? {
                        $(let res: $res =)? $body;
                        $(
                            #[allow(unreachable_code)]
                            let _ = stringify!($res);
                            Ok(res)
                        )?
                    }
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! handler_implementation {
    (
        $(#[$impl_meta:meta])*
        $trait_name:ident as $impl_name:ident {
            $(
                $(#[$method_meta:meta])*
                async fn $method:ident $sig:tt -> $res:ty
                    $body:block
            )*
        }
    ) => {
        $crate::pastey::paste! {
            pub use routes::OpenApi;
            mod routes {
                use super::*;

                #[derive(utoipa::OpenApi)]
                #[openapi(paths($($method),*))]
                pub struct OpenApi;

                $(
                    $(#[$method_meta])*
                    pub async fn $method $sig -> [<$trait_name Result>]<$res> {
                        let res = $body;
                        #[allow(unreachable_code)]
                        Ok(res)
                    }
                )*
            }

            #[derive(Clone)]
            $(#[$impl_meta])*
            pub struct $impl_name;
            impl $trait_name for $impl_name {
                $(
                    #[tracing::instrument(skip_all, level = "trace")]
                    fn $method () -> impl actix_web::dev::HttpServiceFactory + utoipa_actix_web::OpenApiFactory {
                        routes::$method
                    }
                )*
            }

            #[doc(hidden)]
            struct [<$impl_name Helper>];
            #[allow(unused_variables)]
            #[doc(hidden)]
            #[$crate::async_trait::async_trait]
            impl [<$trait_name Helper>] for [<$impl_name Helper>] {
                $(
                    async fn $method $sig -> [<$trait_name Result>]<$res> {
                        let res = $body;
                        #[allow(unreachable_code)]
                        Ok(res)
                    }
                )*
            }
        }
    };
}
