use actix_multipart::form::tempfile::TempFile;
use bytes::Bytes;
use dto::image::Image;
use macros::implementation;
use repository::image::ImageRepositoryDependency;
use ulid::Ulid;

use super::{MAX_IMAGE_SIZE, ProfileImageService, ProfileImageServiceResult};
use crate::{common::ServiceError, profile::ProfileServiceDependency};

#[implementation(result = ProfileImageServiceResult)]
pub mod service {
    struct ProfileImageServiceImpl {
        image_repository: ImageRepositoryDependency,
        profile_service: ProfileServiceDependency,
    }

    impl ProfileImageService for ProfileImageServiceImpl {
        async fn upsert_by_id(
            &self,
            id: Ulid,
            file: TempFile,
            check_user: bool,
        ) -> () {
            let profile =
                self.profile_service.get_by_id(id, check_user).await?;

            if file.size > MAX_IMAGE_SIZE {
                Err(ServiceError::PayloadTooLarge("5.7 MB".into()))?;
            }

            let image_types = ["jpeg", "pjpeg", "png", "webp"];
            let mime_type_err = |type_got: &str| {
                Err(ServiceError::UnsupportedMediaType {
                    supported: image_types
                        .iter()
                        .map(|t| format!("`image/{t}`"))
                        .collect::<Vec<String>>()
                        .join(", "),
                    got: type_got.to_string(),
                })
            };

            let content_type = if let Some(content_type) =
                file.content_type.clone()
            {
                if content_type.type_() != "image"
                    || !image_types.contains(&content_type.subtype().as_str())
                {
                    return mime_type_err(&format!(
                        "{}/{}",
                        content_type.type_(),
                        content_type.subtype()
                    ));
                } else {
                    content_type
                }
            } else {
                return mime_type_err("");
            };

            let data = tokio::fs::read(&file.file.path())
                .await
                .map_err(|err| ServiceError::Internal(err.to_string()))?;

            self.image_repository
                .upsert_by_id(
                    id.into(),
                    Image {
                        content_type: content_type.to_string(),
                        data: Bytes::from(data),
                    }
                    .into(),
                )
                .await?;

            self.profile_service
                .upsert_by_id(
                    profile.id,
                    profile.into(),
                    Some(true),
                    check_user,
                )
                .await?;
        }

        async fn find_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> Option<Image> {
            let profile =
                self.profile_service.get_by_id(id, check_user).await?;

            if !profile.has_avatar {
                return Ok(None);
            }

            self.image_repository
                .find_by_id(id.into())
                .await?
                .map(Image::from)
        }

        async fn get_by_id(&self, id: Ulid, check_user: bool) -> Image {
            self.find_by_id(id, check_user).await?.ok_or(
                ServiceError::NotFound("Profile image with provided id".into()),
            )?
        }

        async fn delete_by_id(&self, id: Ulid, check_user: bool) -> () {
            let profile =
                self.profile_service.get_by_id(id, check_user).await?;
            self.get_by_id(id, false).await?;

            self.image_repository.delete_by_id(id.into()).await?;

            self.profile_service
                .upsert_by_id(profile.id, profile.into(), Some(false), false)
                .await?;
        }
    }
}
