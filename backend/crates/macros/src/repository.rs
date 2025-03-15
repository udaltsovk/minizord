#[macro_export]
macro_rules! repository_entity {
    (
        $item:item 
    ) =>{
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        $item
    };
}

#[macro_export]
macro_rules! repository {
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
            impl [<$name Id>] {
                const TABLE: &str = stringify!([<$name:snake>]);

                #[cfg(feature = "surrealdb")]
                pub fn record_id(&self) -> surrealdb::RecordId {
                    self.0.clone()
                }
            }
            #[cfg(feature = "surrealdb")]
            impl From<$id_ty> for [<$name Id>] {
                fn from(id: $id_ty) -> Self {
                    Self(surrealdb::RecordId::from_table_key(Self::TABLE, id.to_string()))

                }
            }
            #[cfg(feature = "surrealdb")]
            impl ToString for [<$name Id>] {
                fn to_string(&self) -> String {
                    let record_id: surrealdb::RecordId = self.clone().into();
                    record_id.key().to_string()
                }
            }
            #[cfg(feature = "surrealdb")]
            impl Into<surrealdb::RecordId> for [<$name Id>] {
                fn into(self) -> surrealdb::RecordId {
                    self.record_id()
                }
            }
            #[cfg(not(feature = "surrealdb"))]
            impl Into<$id_ty> for [<$name Id>] {
                fn into(self) -> $id_ty {
                    self.0
                }
            }

            $(
                macros::repository_entity! {
                    $(#[$create_meta])*
                    pub struct [<Create $name>] {
                        $(
                            $(#[$create_field_meta])*
                            pub $create_field: $create_ty,
                        )*
                    }
                }
            )?

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

            $(
                macros::repository_entity! {
                    #[derive(Default)]
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

            type [<$name RepositoryResult>]<T> = Result<T, crate::common::RepositoryError>;

            #[macros::async_trait::async_trait]
            pub trait [<$name Repository>] {
                async fn save(&self, new: [<Create $name>]) -> [<$name RepositoryResult>]<$name>;
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
}
