#[macro_export]
macro_rules! derive_entity {
    (
        $item:item
    ) => {
        #[derive(
            serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq,
        )]
        $item
    };
}

#[macro_export]
macro_rules! entity {
    ($(
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
                $(#[$upsert_meta:meta])*
                upsert {
                    $(
                        $(#[$upsert_field_meta:meta])*
                        $upsert_field:ident: $upsert_ty:ty
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
    )*) => {
        $crate::paste::paste! {$(
            #[cfg(feature = "surrealdb")]
            macros::derive_entity! {
                #[serde(transparent)]
                pub struct [<$name Id>](surrealdb::RecordId);
            }
            #[cfg(not(feature = "surrealdb"))]
            macros::derive_entity! {
                #[serde(transparent)]
                pub struct [<$name Id>]($id_ty);
            }
            impl macros::RepositoryId for [<$name Id>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                #[cfg(feature = "surrealdb")]
                #[tracing::instrument(skip_all, level = "trace")]
                fn record_id(&self) -> surrealdb::RecordId {
                    self.0.clone()
                }
            }
            #[cfg(feature = "surrealdb")]
            impl From<$id_ty> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn from(id: $id_ty) -> Self {
                    Self(surrealdb::RecordId::from_table_key(Self::TABLE, id.to_string()))

                }
            }
            #[cfg(feature = "surrealdb")]
            impl std::fmt::Display for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    let record_id: surrealdb::RecordId = self.clone().into();
                    record_id.key().fmt(formatter)
                }
            }
            #[cfg(feature = "surrealdb")]
            impl From<[<$name Id>]> for surrealdb::RecordId {
                #[tracing::instrument(skip_all, level = "trace")]
                fn from(id: [<$name Id>]) -> Self {
                    id.record_id()
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl std::fmt::Display for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.0.fmt(formatter)
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl From<$id_ty> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn from(id: $id_ty) -> Self {
                    Self(id)
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl From<[<$name Id>]> for $id_ty {
                #[tracing::instrument(skip_all, level = "trace")]
                fn from(id: [<$name Id>]) -> Self {
                    id.0
                }
            }

            macros::derive_entity! {
                $($(#[$create_meta])*)?
                pub struct [<Create $name>] {
                    $($(
                        $(#[$create_field_meta])*
                        pub $create_field: $create_ty,
                    )*)?
                }
            }

            macros::derive_entity! {
                $($(#[$upsert_meta])*)?
                pub struct [<Upsert $name>] {
                    $($(
                        $(#[$upsert_field_meta])*
                        pub $upsert_field: $upsert_ty,
                    )*)?
                }
            }

            macros::derive_entity! {
                $(#[$meta])*
                pub struct $name {
                    pub id: [<$name Id>],
                    $(
                        $(
                            $(#[$field_meta])*
                            pub $field: $ty,
                        )*
                    )?
                }
            }

            macros::derive_entity! {
                #[derive(Default)]
                $($(#[$update_meta])*)?
                pub struct [<$name Update>] {
                    $($(
                        #[serde(skip_serializing_if = "Option::is_none")]
                        $(#[$update_field_meta])*
                        pub $update_field: Option<$update_ty>,
                    )*)?
                }
            }
        )*}
    };
    (
        $(#[$meta:meta])*
        $in:ident -> $name:ident -> $out:ident {
            $(
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
                $(#[$upsert_meta:meta])*
                upsert {
                    $(
                        $(#[$upsert_field_meta:meta])*
                        $upsert_field:ident: $upsert_ty:ty
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
        $crate::paste::paste! {
            macros::derive_entity! {
                $($(#[$create_meta])*)?
                pub struct [<Create $name>] {
                    pub r#in: $in,
                    pub out: $out,
                    $($(
                        $(#[$create_field_meta])*
                        pub $create_field: $create_ty,
                    )*)?
                }

            }
            impl [<Create $name>] {
                fn get_id_string(&self) -> String {
                    format!("{}_{}", self.r#in, self.out)
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(&self, table: &'static str) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(table, self.get_id_string())
                }
            }

            macros::derive_entity! {
                $($(#[$upsert_meta])*)?
                pub struct [<Upsert $name>] {
                    pub r#in: $in,
                    pub out: $out,
                    $($(
                        $(#[$upsert_field_meta])*
                        pub $upsert_field: $upsert_ty,
                    )*)?
                }

            }
            impl [<Upsert $name>] {
                fn get_id_string(&self) -> String {
                    format!("{}_{}", self.r#in, self.out)
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(&self, table: &'static str) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(table, self.get_id_string())
                }
            }

            macros::derive_entity! {
                $(#[$meta])*
                pub struct $name {
                    pub id: String,
                    pub r#in: $in,
                    pub out: $out,
                    $($(
                        $(#[$field_meta])*
                        pub $field: $ty,
                    )*)?
                }
            }

            macros::derive_entity! {
                #[derive(Default)]
                $($(#[$update_meta])*)?
                pub struct [<$name Update>] {
                    $($(
                        #[serde(skip_serializing_if = "Option::is_none")]
                        $(#[$update_field_meta])*
                        pub $update_field: Option<$update_ty>,
                    )*)?
                }
            }
        }
    };
}
