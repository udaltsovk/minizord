use actix_multipart::form::tempfile::TempFile;
use bytes::Bytes;
use dto::{
    image::Image,
    profile::{Profile, UpsertProfile},
};
use entity::{
    profile::{ProfileId, UpsertProfile as UpsertProfileEntity},
    user::UserUpdate as UserEntityUpdate,
};
use macros::implementation;
use repository::{
    image::ImageRepositoryDependency, profile::ProfileRepositoryDependency,
    user::UserRepositoryDependency,
};
use ulid::Ulid;

use super::{ProfileService, ProfileServiceResult};
use crate::common::ServiceError;

const MAX_IMAGE_SIZE: usize = 5_976_883;

implementation! {
    ProfileService {
        user_repository: UserRepositoryDependency,
        profile_repository: ProfileRepositoryDependency,
        image_repository: ImageRepositoryDependency,
    } as Implemented {
        upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
        ) -> Profile {
            let profile_id: ProfileId = id.into();
            let profile = self.profile_repository
                .find_by_id(profile_id.clone())
                .await?;
            let profile = self.profile_repository
                .upsert_by_id(
                    profile_id.clone(),
                    UpsertProfileEntity {
                        name: object.name,
                        surname: object.surname,
                        telegram: object.telegram,
                        city: object.city,
                        bio: object.bio,
                        portfolio_urls: object.portfolio_urls.clone(),
                        has_avatar: profile.map(|p| p.has_avatar).unwrap_or(false),
                    }
                )
                .await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(Some(profile_id.clone())),
                        ..Default::default()
                    }
                )
                .await?;
            profile.into()
        }

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<Profile> {
            self.profile_repository
                .find_by_id(id.into())
                .await?
                .map(Profile::from)
        }

        get_by_id(
            &self,
            id: Ulid,
        ) -> Profile {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Profile with provided id".into())
                )?
        }

        delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            self.get_by_id(id).await?;
            self.profile_repository
                .delete_by_id(id.into())
                .await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(None),
                        ..Default::default()
                    }
                )
                .await?;
        }

        upsert_image_by_id(
            &self,
            id: Ulid,
            file: TempFile,
        ) -> () {
            let mut profile = self.get_by_id(id).await?;

            if file.size > MAX_IMAGE_SIZE {
                Err(ServiceError::PayloadTooLarge("5.7 MB".into()))?;
            }

            let image_types = ["jpeg", "pjpeg", "png", "webp"];
            let mime_type_err = |type_got: &str| Err(ServiceError::UnsupportedMediaType {
                supported: image_types.iter().map(|t| format!("`image/{t}`")).collect::<Vec<String>>().join(", "),
                got: type_got.to_string()
            });

            let content_type = if let Some(content_type) = file.content_type.clone() {
                if content_type.type_() != "image"
                    || !image_types.contains(&content_type.subtype().as_str())
                {
                    return mime_type_err(&format!("{}/{}", content_type.type_(), content_type.subtype()));
                } else {
                    content_type
                }
            } else {
                return mime_type_err("");
            };

            let data = tokio::fs::read(&file.file.path()).await.map_err(|err| ServiceError::Internal(err.to_string()))?;

            self.image_repository
                .upsert_by_id(
                    id.into(),
                    Image {
                        content_type: content_type.to_string(),
                        data: Bytes::from(data),
                    }.into(),
                )
                .await?;

            profile.has_avatar = true;
            self.profile_repository
                .upsert_by_id(
                    profile.id.into(),
                    profile.into(),
                )
                .await?;
        }

        find_image_by_id(
            &self,
            id: Ulid,
        ) -> Option<Image> {
            let profile = self.get_by_id(id).await?;

            if !profile.has_avatar {
                return Ok(None);
            }

            self.image_repository
                .find_by_id(id.into())
                .await?
                .map(Image::from)
        }

        get_image_by_id(
            &self,
            id: Ulid,
        ) -> Image {
            self
                .find_image_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Profile image with provided id".into())
                )?
        }

        delete_image_by_id(
            &self,
            id: Ulid,
        ) -> () {
            let mut profile = self.get_by_id(id).await?;
            self.get_image_by_id(id).await?;

            self.image_repository
                .delete_by_id(id.into())
                .await?;

            profile.has_avatar = false;
            self.profile_repository
                .upsert_by_id(
                    profile.id.into(),
                    profile.into()
                )
                .await?;
        }
    }
}
