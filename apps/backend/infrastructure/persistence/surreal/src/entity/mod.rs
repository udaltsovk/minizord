use std::marker::PhantomData;

use derive_where::derive_where;
use lib::domain::Id;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::{
    RecordId,
    opt::{IntoResource, Resource},
};
use tap::Pipe as _;
use uuid::Uuid;

pub(crate) mod profile;
pub(crate) mod user;

#[derive_where(Clone, Debug)]
pub struct SurrealId<T> {
    pub value: RecordId,
    _entity: PhantomData<T>,
}

pub trait SurrealIdWithTable {
    const TABLE: &str;
}

#[macro_export]
macro_rules! impl_table_for {
    ($type: ident) => {
        impl $crate::entity::SurrealIdWithTable
            for $crate::entity::SurrealId<$type>
        {
            const TABLE: &str =
                pastey::paste!(stringify!([<$type:snake:lower>]));
        }
    };
}

impl<'de, T> Deserialize<'de> for SurrealId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self {
            value: RecordId::deserialize(deserializer)?,
            _entity: PhantomData,
        }
        .pipe(Ok)
    }
}

impl<T> Serialize for SurrealId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<T> From<Id<T>> for SurrealId<T>
where
    SurrealId<T>: SurrealIdWithTable,
{
    fn from(id: Id<T>) -> Self {
        Self {
            value: RecordId::from_table_key(Self::TABLE, id.value),
            _entity: PhantomData,
        }
    }
}

impl<T> From<SurrealId<T>> for Id<T> {
    fn from(id: SurrealId<T>) -> Self {
        id.value
            .key()
            .to_string()
            .strip_prefix("u'")
            .expect("stripped uuid")
            .strip_suffix("'")
            .expect("stripped uuid")
            .pipe(Uuid::parse_str)
            .expect("A valid UUIDV7")
            .into()
    }
}

impl<R, T> IntoResource<Option<R>> for SurrealId<T> {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(self.value.into())
    }
}
