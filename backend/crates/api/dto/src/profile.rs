use entity::profile::{
    Profile as ProfileEntity, UpsertProfile as UpsertProfileEntity,
};
use macros::dto;
use ulid::Ulid;
use utils::validation::{
    RE_NAME, RE_TELEGRAM_USERNAME, validate_portfolio_url,
};

dto! {
    ///
    Profile {
        fields {
            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            id: Ulid,

            ///
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]
            name: String,

            ///
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]
            surname: String,

            ///
            #[schema(min_length = 5, max_length = 32, pattern = r#"^[A-Za-z\d_]{5,32}$"#)]
            telegram: String,

            //
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]
            city: String,

            ///
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,

            ///
            #[schema(min_length = 0)]
            portfolio_urls: Vec<String>,

            ///
            has_avatar: bool,
        },
        upsert
        ///
        {
            ///
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]
            #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
            name: String,

            ///
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]
            #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
            surname: String,

            ///
            #[schema(min_length = 5, max_length = 32, pattern = r#"^[A-Za-z\d_]{5,32}$"#)]
            #[garde(length(min = 5, max = 32), pattern(*RE_TELEGRAM_USERNAME))]
            telegram: String,

            //
            #[schema(
                min_length = 2,
                max_length = 24,
                pattern = r#"(^[А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*(?: [А-ЯЁ][а-яё]*(?:['-][А-ЯЁ][а-яё]*)*)*$)"#
            )]

            #[garde(length(min = 2, max = 24), pattern(*RE_NAME))]
            city: String,

            ///
            #[schema(min_length = 0, max_length = 4096)]
            #[garde(length(min = 0, max = 4096))]
            bio: String,

            ///
            #[schema(min_length = 0)]
            #[garde(
                length(min = 0),
                inner(url, custom(validate_portfolio_url)),
            )]
            portfolio_urls: Vec<String>,
        },
    }
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
