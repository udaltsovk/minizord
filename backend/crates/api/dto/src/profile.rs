use entity::profile::{
    Profile as ProfileEntity, UpsertProfile as UpsertProfileEntity,
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utils::validation::{
    RE_NAME, RE_TELEGRAM_USERNAME, validate_portfolio_url,
};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Debug)]
///
pub struct Profile {
    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub id: Ulid,

    ///
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    pub name: String,

    ///
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    pub surname: String,

    ///
    #[schema(
        min_length = 5,
        max_length = 32,
        pattern = r#"^[A-Za-z\d_]{5,32}$"#
    )]
    pub telegram: String,

    //
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    pub city: String,

    ///
    #[schema(min_length = 0, max_length = 4096)]
    pub bio: String,

    ///
    #[schema(min_length = 0)]
    pub portfolio_urls: Vec<String>,

    ///
    pub has_avatar: bool,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct UpsertProfile {
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
    pub name: String,

    ///
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
    pub surname: String,

    ///
    #[schema(
        min_length = 5,
        max_length = 32,
        pattern = r#"^[A-Za-z\d_]{5,32}$"#
    )]
    #[garde(length(min = 5, max = 32), pattern(*RE_TELEGRAM_USERNAME))]
    pub telegram: String,

    //
    #[schema(
        min_length = 2,
        max_length = 24,
        pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
    )]
    #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
    pub city: String,

    ///
    #[schema(min_length = 0, max_length = 4096)]
    #[garde(length(min = 0, max = 4096))]
    pub bio: String,

    ///
    #[schema(min_length = 0)]
    #[garde(length(min = 0), inner(url, custom(validate_portfolio_url)))]
    pub portfolio_urls: Vec<String>,
}

impl From<Profile> for UpsertProfile {
    fn from(dto: Profile) -> Self {
        Self {
            name: dto.name,
            surname: dto.surname,
            telegram: dto.telegram,
            city: dto.city,
            bio: dto.bio,
            portfolio_urls: dto.portfolio_urls,
        }
    }
}
impl From<ProfileEntity> for Profile {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: ProfileEntity) -> Self {
        Self {
            id: entity.id.into(),
            name: entity.name,
            surname: entity.surname,
            telegram: entity.telegram,
            city: entity.city,
            bio: entity.bio,
            portfolio_urls: entity.portfolio_urls,
            has_avatar: entity.has_avatar,
        }
    }
}
impl From<Profile> for UpsertProfileEntity {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(dto: Profile) -> Self {
        Self {
            name: dto.name,
            surname: dto.surname,
            telegram: dto.telegram,
            city: dto.city,
            bio: dto.bio,
            portfolio_urls: dto.portfolio_urls,
            has_avatar: dto.has_avatar,
        }
    }
}
