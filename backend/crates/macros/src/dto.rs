#[macro_export]
macro_rules! derive_dto {
    (
        $(#[$meta:meta])*
        $item:item
    ) => {
        #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, validator::Validate, Debug, Clone, PartialEq)]
        $(#[$meta])*
        $item
    };
}

#[macro_export]
macro_rules! dto {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        macros::paste::paste! {
            macros::derive_dto! {
                $(#[$meta])*
                pub struct $name {
                    $(
                        $(#[$field_meta])*
                        pub $field: $ty,
                    )*
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident {
            id: $id_ty:ty
            $(
                ,
                fields {
                    $(
                        $(#[$field_meta:meta])*
                        $field:ident: $ty:ty
                    ),* $(,)?
                }
            )?
            $(
                ,
                $(#[$create_meta:meta])*
                create {
                    $(
                        $(#[$create_field_meta:meta])*
                        $create_field:ident: $create_ty:ty
                    ),* $(,)?
                }
            )?
            $(
                ,
                $(#[$update_meta:meta])*
                update {
                    $(
                        $(#[$update_field_meta:meta])*
                        $update_field:ident: $update_ty:ty
                    ),* $(,)?
                }
            )? $(,)?
        }
    ) => {
        macros::paste::paste! {
            $(
                macros::derive_dto! {
                    $(#[$create_meta])*
                    pub struct [<Create $name>] {
                        $(
                            $(#[$create_field_meta])*
                            pub $create_field: $create_ty,
                        )*
                    }
                }
            )?

            macros::derive_dto! {
                $(#[$meta])*
                pub struct $name {
                    pub id: $id_ty,
                    $(
                        $(
                            $(#[$field_meta])*
                            pub $field: $ty,
                        )*
                    )?
                }
            }

            $(
                macros::derive_dto! {
                    $(#[$update_meta])*
                    pub struct [<$name Update>] {
                        $(
                            #[serde(skip_serializing_if = "Option::is_none")]
                            $(#[$update_field_meta])*
                            pub $update_field: Option<$update_ty>,
                        )*
                    }
                }
            )?
        }
    };
}
