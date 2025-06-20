use aws_sdk_s3::{
    operation::{get_object::GetObjectError, head_object::HeadObjectError},
    primitives::ByteStream,
};
use entity::image::{Image, ImageId, UpsertImage};
use macros::implementation;
use utils::adapters::S3;

use super::{ImageRepository, ImageRepositoryResult};
use crate::common::RepositoryError;

#[tracing::instrument(level = "trace")]
fn mime_extension(content_type: &str) -> String {
    content_type
        .split('/')
        .nth(1)
        .map(|ext| {
            match ext {
                "jpeg" => "jpg",
                "svg+xml" => "svg",
                _ => ext,
            }
            .to_string()
        })
        .unwrap_or_else(|| "bin".to_string())
}

#[implementation(result = ImageRepositoryResult)]
pub mod repository {
    pub struct S3ImageRepository {
        s3: S3,
    }

    impl ImageRepository for S3ImageRepository {
        async fn upsert_by_id(
            &self,
            id: ImageId,
            object: UpsertImage,
        ) -> Image {
            let content_type = object.content_type.clone();
            self.s3
                .put_object()
                .bucket(ImageId::BUCKET)
                .key(id.to_string())
                .content_type(content_type.clone())
                .metadata(
                    "filename",
                    format!("{}.{}", id, mime_extension(&content_type)),
                )
                .body(ByteStream::from(object.data.clone()))
                .send()
                .await
                .map_err(aws_sdk_s3::Error::from)?;

            Image {
                id,
                content_type,
                data: object.data,
            }
        }

        async fn find_by_id(&self, id: ImageId) -> Option<Image> {
            let response = match self
                .s3
                .get_object()
                .bucket(ImageId::BUCKET)
                .key(id.to_string())
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(error) => match error.as_service_error() {
                    Some(GetObjectError::NoSuchKey(..)) => return Ok(None),
                    _ => Err(aws_sdk_s3::Error::from(error))?,
                },
            };

            let content_type = response
                .content_type()
                .ok_or(RepositoryError::S3BrokenImage)?
                .to_string();

            let data = response.body.collect().await?.into_bytes();

            Some(Image {
                id,
                content_type,
                data,
            })
        }

        async fn exists_by_id(&self, id: ImageId) -> bool {
            match self
                .s3
                .head_object()
                .bucket(ImageId::BUCKET)
                .key(id.to_string())
                .send()
                .await
            {
                Ok(..) => true,
                Err(error) => match error.as_service_error() {
                    Some(HeadObjectError::NotFound(..)) => return Ok(false),
                    _ => Err(aws_sdk_s3::Error::from(error))?,
                },
            }
        }

        async fn delete_by_id(&self, id: ImageId) -> Option<Image> {
            let image = self.find_by_id(id.clone()).await?;
            if image.is_some() {
                self.s3
                    .delete_object()
                    .bucket(ImageId::BUCKET)
                    .key(id.to_string())
                    .send()
                    .await
                    .map_err(aws_sdk_s3::Error::from)?;
            }
            image
        }
    }
}
