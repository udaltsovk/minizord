use bytes::Bytes;
use entity::image::{Image as ImageEntity, UpsertImage as UpsertImageEntity};
use macros::dto;

dto! {
    ///
    Image {
        fields {
            ///
            #[garde(skip)]
            content_type: String,
            ///
            #[schema(value_type = String, format = Binary)]
            #[garde(skip)]
            data: Bytes,
        },
    }
}

impl From<ImageEntity> for Image {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: ImageEntity) -> Self {
        Self {
            content_type: entity.content_type,
            data: entity.data,
        }
    }
}
impl From<Image> for UpsertImageEntity {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(dto: Image) -> Self {
        Self {
            content_type: dto.content_type,
            data: dto.data,
        }
    }
}
