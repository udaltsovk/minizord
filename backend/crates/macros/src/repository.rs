pub trait RepositoryId: std::fmt::Display {
    const TABLE: &str;

    #[cfg(feature = "surrealdb")]
    fn record_id(&self) -> surrealdb::RecordId;
}

#[macro_export]
macro_rules! repository_entity {
    ( $item:item ) => {
        #[derive(
            serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq,
        )]
        $item
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! crud_repository {
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
        } $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        macros::paste::paste! {
            #[cfg(feature = "surrealdb")]
            macros::repository_entity! {
                #[serde(transparent)]
                pub struct [<$name Id>](surrealdb::RecordId);
            }
            #[cfg(not(feature = "surrealdb"))]
            macros::repository_entity! {
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
            impl Into<surrealdb::RecordId> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn into(self) -> surrealdb::RecordId {
                    self.record_id()
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl Into<$id_ty> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn into(self) -> $id_ty {
                    self.0
                }
            }

            macros::repository_entity! {
                $($(#[$create_meta])*)?
                pub struct [<Create $name>] {
                    $($(
                        $(#[$create_field_meta])*
                        pub $create_field: $create_ty,
                    )*)?
                }
            }

            macros::repository_entity! {
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

            macros::repository_entity! {
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

            type [<$name RepositoryResult>]<T> = Result<T, crate::common::RepositoryError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Repository>] {
                async fn save(&self, object: [<Create $name>]) -> [<$name RepositoryResult>]<$name>;
                async fn find_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn exists_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<bool>;
                async fn update_by_id(&self, id: [<$name Id>], update: [<$name Update>]) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn delete_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<Option<$name>>;
                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name RepositoryDependency>] = std::sync::Arc<dyn [<$name Repository>] + Send + Sync>;
        }
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
                $(#[$update_meta:meta])*
                update {
                    $(
                        $(#[$update_field_meta:meta])*
                        $update_field:ident: $update_ty:ty
                    ),* $(,)?
                }
            )? $(,)?
        } $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        macros::paste::paste! {
            macros::repository_entity! {
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

            macros::repository_entity! {
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

            macros::repository_entity! {
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

            type [<$name RepositoryResult>]<T> = Result<T, crate::common::RepositoryError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Repository>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                fn get_id_string(in_id: &$in, out_id: &$out) -> String {
                    format!("{in_id}_{out_id}")
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(in_id: &$in, out_id: &$out) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(Self::TABLE, Self::get_id_string(in_id, out_id))
                }

                async fn save(&self, object: [<Create $name>]) -> [<$name RepositoryResult>]<$name>;
                async fn find_all_by_in(&self, r#in: $in, limit: u64, offset: u64) -> [<$name RepositoryResult>]<Vec<$name>>;
                async fn exists_by_in(&self, r#in: $in) -> [<$name RepositoryResult>]<bool>;
                async fn find_all_by_out(&self, out: $out, limit: u64, offset: u64) -> [<$name RepositoryResult>]<Vec<$name>>;
                async fn exists_by_out(&self, out: $out) -> [<$name RepositoryResult>]<bool>;
                async fn find_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn exists_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<bool>;
                async fn update_by_in_and_out(&self, r#in: $in, out: $out, update: [<$name Update>]) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn delete_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<Option<$name>>;
                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name RepositoryDependency>] = std::sync::Arc<dyn [<$name Repository>] + Send + Sync>;
        }
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! urd_repository {
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
                $(#[$upsert_meta:meta])*
                upsert {
                    $(
                        $(#[$upsert_field_meta:meta])*
                        $upsert_field:ident: $upsert_ty:ty
                    ),* $(,)?
                }
            )? $(,)?
        } $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        macros::paste::paste! {
            #[cfg(feature = "surrealdb")]
            macros::repository_entity! {
                #[serde(transparent)]
                pub struct [<$name Id>](surrealdb::RecordId);
            }
            #[cfg(not(feature = "surrealdb"))]
            macros::repository_entity! {
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
            impl Into<surrealdb::RecordId> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn into(self) -> surrealdb::RecordId {
                    self.record_id()
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl Into<$id_ty> for [<$name Id>] {
                #[tracing::instrument(skip_all, level = "trace")]
                fn into(self) -> $id_ty {
                    self.0
                }
            }

            macros::repository_entity! {
                $($(#[$upsert_meta])*)?
                pub struct [<Upsert $name>] {
                    $($(
                        $(#[$upsert_field_meta])*
                        pub $upsert_field: $upsert_ty,
                    )*)?
                }
            }

            macros::repository_entity! {
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

            type [<$name RepositoryResult>]<T> = Result<T, crate::common::RepositoryError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Repository>] {
                async fn upsert_by_id(&self, id: [<$name Id>], object: [<Upsert $name>]) -> [<$name RepositoryResult>]<$name>;
                async fn find_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn exists_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<bool>;
                async fn delete_by_id(&self, id: [<$name Id>]) -> [<$name RepositoryResult>]<Option<$name>>;
                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name RepositoryDependency>] = std::sync::Arc<dyn [<$name Repository>] + Send + Sync>;
        }
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
                $(#[$upsert_meta:meta])*
                upsert {
                    $(
                        $(#[$upsert_field_meta:meta])*
                        $upsert_field:ident: $upsert_ty:ty
                    ),* $(,)?
                }
            )? $(,)?
        } $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        macros::paste::paste! {
            macros::repository_entity! {
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

            macros::repository_entity! {
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

            type [<$name RepositoryResult>]<T> = Result<T, crate::common::RepositoryError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Repository>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                fn get_id_string(in_id: &$in, out_id: &$out) -> String {
                    format!("{in_id}_{out_id}")
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(in_id: &$in, out_id: &$out) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(Self::TABLE, Self::get_id_string(in_id, out_id))
                }

                async fn upsert_by_in_and_out(&self, r#in: $in, out: $out, object: [<Upsert $name>]) -> [<$name RepositoryResult>]<$name>;
                async fn find_all_by_in(&self, r#in: $in, limit: u64, offset: u64) -> [<$name RepositoryResult>]<Vec<$name>>;
                async fn exists_by_in(&self, r#in: $in) -> [<$name RepositoryResult>]<bool>;
                async fn find_all_by_out(&self, out: $out, limit: u64, offset: u64) -> [<$name RepositoryResult>]<Vec<$name>>;
                async fn exists_by_out(&self, out: $out) -> [<$name RepositoryResult>]<bool>;
                async fn find_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<Option<$name>>;
                async fn exists_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<bool>;
                async fn delete_by_in_and_out(&self, r#in: $in, out: $out) -> [<$name RepositoryResult>]<Option<$name>>;
                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name RepositoryDependency>] = std::sync::Arc<dyn [<$name Repository>] + Send + Sync>;
        }
    };
}
