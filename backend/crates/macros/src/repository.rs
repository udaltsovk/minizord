pub trait RepositoryId: std::fmt::Display {
    const TABLE: &str;

    #[cfg(feature = "surrealdb")]
    fn record_id(&self) -> surrealdb::RecordId;
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! crud_repository {
    (
        $(#[$meta:meta])*
        $name:ident
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::paste::paste! {
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
        $in:ident -> $name:ident -> $out:ident
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::paste::paste! {
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
        $name:ident $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::paste::paste! {
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
        $in:ident -> $name:ident -> $out:ident
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::paste::paste! {
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
