#[macro_export]
macro_rules! request {
    (
        $(
            $(#[$meta:meta])*
            $name:ident {
                $(
                    $(#[$field_meta:meta])*
                    $field:ident: $ty:ty
                ),* $(,)?
            } $(,)?
        )*
    ) => {
        $crate::pastey::paste! {
            $(
                #[derive(serde::Deserialize, utoipa::ToSchema, validator::Validate, Debug, Clone)]
                $(#[$meta])*
                pub struct [<$name Request>] {
                    $(
                        $(#[$field_meta])*
                        pub $field: $ty,
                    )*
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! response {
    (
        $(
            $(#[$meta:meta])*
            $name:ident {
                $(
                    $(#[$field_meta:meta])*
                    $field:ident: $ty:ty
                ),* $(,)?
            } $(,)?
        )*
    ) => {
        $crate::pastey::paste! {
            $(
                #[derive(serde::Serialize, utoipa::ToSchema, Debug, Clone)]
                $(#[$meta])*
                pub struct [<$name Response>] {
                    $(
                        $(#[$field_meta])*
                        pub $field: $ty,
                    )*
                }
            )*
        }
    };
}
