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
                $method_vis:vis $method:ident $sig:tt $(-> $res:ty)?
                    $body:block
            )*
        }
    ) => {
        macros::paste::paste! {
            $(#[$impl_meta])*
            pub struct [<$impl_name $trait_name>] {
                $($(
                    $(#[$field_meta])*
                    $field_vis $field: $ty,
                )*)?
            }

            #[macros::async_trait::async_trait]
            impl super::$trait_name for [<$impl_name $trait_name>] {
                $(
                    $(#[$method_meta])*
                    $method_vis fn $method $sig $(-> Result<$res, [<$trait_name Error>]>)? {
                        $body
                    }
                )*
            }
        }
    };
}
