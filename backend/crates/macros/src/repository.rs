#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! crud_repository {
    (
        $(#[$meta:meta])*
        $name:ident
            Err: $err_ty:ty
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::pastey::paste! {
            type [<$name:camel RepositoryResult>]<T> = Result<T, $err_ty>;

            #[$crate::async_trait::async_trait]
            pub trait [<$name:camel Repository>] {
                async fn save(
                    &self,
                    object: [<$name:snake>]::[<Create $name:camel>]
                ) -> [<$name:camel RepositoryResult>]<[<$name:snake>]::[<$name:camel>]>;

                async fn find_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn update_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>],
                    update: [<$name:snake>]::[<$name:camel Update>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn delete_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name:camel RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name:camel RepositoryDependency>] = std::sync::Arc<dyn [<$name:camel Repository>] + Send + Sync>;
        }
    };
    (
        $(#[$meta:meta])*
        $in:ident -> $name:ident -> $out:ident
            Err: $err_ty:ty
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::pastey::paste! {
            type [<$name:camel RepositoryResult>]<T> = Result<T, $err_ty>;

            #[$crate::async_trait::async_trait]
            pub trait [<$name:camel Repository>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                fn get_id_string(in_id: &$in, out_id: &$out) -> String {
                    format!("{in_id}_{out_id}")
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(in_id: &$in, out_id: &$out) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(Self::TABLE, Self::get_id_string(in_id, out_id))
                }

                async fn save(
                    &self,
                    object: [<$name:snake>]::[<Create $name:camel>]
                ) -> [<$name:camel RepositoryResult>]<[<$name:snake>]::[<$name:camel>]>;

                async fn find_all_by_in(
                    &self,
                    r#in: $in,
                    limit: u64,
                    offset: u64
                ) -> [<$name:camel RepositoryResult>]<Vec<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_in(
                    &self,
                    r#in: $in
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn find_all_by_out(
                    &self,
                    out: $out,
                    limit: u64,
                    offset: u64
                ) -> [<$name:camel RepositoryResult>]<Vec<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_out(
                    &self,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn find_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn update_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out,
                    update: [<$name:snake>]::[<$name:camel Update>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn delete_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name:camel RepositoryDependency>] = std::sync::Arc<dyn [<$name:camel Repository>] + Send + Sync>;
        }
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! urd_repository {
    (
        $(#[$meta:meta])*
        $name:ident
            Err: $err_ty:ty
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::pastey::paste! {
            type [<$name:camel RepositoryResult>]<T> = Result<T, $err_ty>;

            #[$crate::async_trait::async_trait]
            pub trait [<$name:camel Repository>] {
                async fn upsert_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>],
                    object: [<$name:snake>]::[<Upsert $name:camel>]
                ) -> [<$name:camel RepositoryResult>]<[<$name:snake>]::[<$name:camel>]>;

                async fn find_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn delete_by_id(
                    &self,
                    id: [<$name:snake>]::[<$name:camel Id>]
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name:camel RepositoryDependency>] = std::sync::Arc<dyn [<$name:camel Repository>] + Send + Sync>;
        }
    };
    (
        $(#[$meta:meta])*
        $in:ident -> $name:ident -> $out:ident
            Err: $err_ty:ty
        $({
            $(
                $(#[$fn_meta:meta])*
                $method:ident $sig:tt -> $res:ty;
            )*
        })?
    ) => {
        $crate::pastey::paste! {
            type [<$name:camel RepositoryResult>]<T> = Result<T, $err_ty>;

            #[$crate::async_trait::async_trait]
            pub trait [<$name:camel Repository>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                fn get_id_string(in_id: &$in, out_id: &$out) -> String {
                    format!("{in_id}_{out_id}")
                }

                #[cfg(feature = "surrealdb")]
                fn get_id(in_id: &$in, out_id: &$out) -> surrealdb::RecordId {
                    surrealdb::RecordId::from_table_key(Self::TABLE, Self::get_id_string(in_id, out_id))
                }

                async fn upsert_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out,
                    object: [<$name:snake>]::[<Upsert $name:camel>]
                ) -> [<$name:camel RepositoryResult>]<[<$name:snake>]::[<$name:camel>]>;

                async fn find_all_by_in(
                    &self,
                    r#in: $in,
                    limit: u64,
                    offset: u64
                ) -> [<$name:camel RepositoryResult>]<Vec<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_in(
                    &self,
                    r#in: $in
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn find_all_by_out(
                    &self,
                    out: $out,
                    limit: u64,
                    offset: u64
                ) -> [<$name:camel RepositoryResult>]<Vec<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_out(
                    &self,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn find_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                async fn exists_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<bool>;

                async fn delete_by_in_and_out(
                    &self,
                    r#in: $in,
                    out: $out
                ) -> [<$name:camel RepositoryResult>]<Option<[<$name:snake>]::[<$name:camel>]>>;

                $(
                    $(
                        $(#[$fn_meta])*
                        async fn $method $sig -> [<$name:camel RepositoryResult>]<$res>;
                    )*
                )?
            }

            pub type [<$name:camel RepositoryDependency>] = std::sync::Arc<dyn [<$name:camel Repository>] + Send + Sync>;
        }
    };
}
