use bytes::Bytes;
use macros::dto;
use repository::image::{
    Image as ImageEntity, UpsertImage as UpsertImageEntity,
};

dto! {
    ///
    Image {
        ///
        content_type: String,
        ///
        #[schema(value_type = String, format = Binary)]
        data: Bytes,
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
