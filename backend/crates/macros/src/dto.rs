#[macro_export]
macro_rules! derive_dto {
    (
        $item:item
    ) => {
        #[derive(
            serde::Serialize,
            serde::Deserialize,
            utoipa::ToSchema,
            garde::Validate,
            Clone,
            PartialEq,
            Debug,
        )]
        $item
    };
}

#[macro_export]
macro_rules! dto {
    ($(
        $(#[$meta:meta])*
        $name:ident {
            $(
                fields {
                    $(
                        $(#[$field_meta:meta])*
                        $field:ident: $ty:ty
                    ),* $(,)?
                } $(,)?
            )?
            $(
                create
                $(#[$create_meta:meta])*
                {
                    $(
                        $(#[$create_field_meta:meta])*
                        $create_field:ident: $create_ty:ty
                    ),* $(,)?
                } $(,)?
            )?
            $(
                upsert
                $(#[$upsert_meta:meta])*
                {
                    $(
                        $(#[$upsert_field_meta:meta])*
                        $upsert_field:ident: $upsert_ty:ty
                    ),* $(,)?
                } $(,)?
            )?
            $(
                update
                $(#[$update_meta:meta])*
                {
                    $(
                        $(#[$update_field_meta:meta])*
                        $update_field:ident: $update_ty:ty
                    ),* $(,)?
                } $(,)?
            )?
        }
    )*) => {
        $crate::pastey::paste! {$(
            $($crate::derive_dto! {
                $(#[$create_meta])*
                pub struct [<Create $name>] {
                    $(
                        $(#[$create_field_meta])*
                        pub $create_field: $create_ty,
                    )*
                }
            })?
            $($crate::derive_dto! {
                $(#[$upsert_meta])*
                pub struct [<Upsert $name>] {
                    $(
                        $(#[$upsert_field_meta])*
                        pub $upsert_field: $upsert_ty,
                    )*
                }
            })?
            $crate::derive_dto! {
                $(#[$meta])*
                pub struct $name {
                    $(
                        $(
                            $(#[$field_meta])*
                            pub $field: $ty,
                        )*
                    )?
                }
            }
            $($crate::derive_dto! {
                $(#[$update_meta])*
                pub struct [<$name Update>] {
                    $(
                        #[serde(skip_serializing_if = "Option::is_none")]
                        $(#[$update_field_meta])*
                        pub $update_field: Option<$update_ty>,
                    )*
                }
            })?
        )*}
    };
}
