use actix_web::{
    HttpResponse, delete, get, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::{Path, Query};
use actix_web_validation::Validated;
use dto::{
    Pagination,
    review::{Review, UpsertReview},
    user::User,
};
use macros::implementation;
use service::review::ReviewServiceDependency;
use ulid::Ulid;

use super::{ReviewHandler, ReviewHandlerResult};
use crate::common::{ApiError, ValidationError, openapi};

#[implementation(
    r#trait = ReviewHandler,
    name = ReviewHandlerImpl,
    result = ReviewHandlerResult,
)]
pub mod handler {
    ///
    #[openapi(
            params(
                ("reviewee_id" = Ulid, description = ""),
                Pagination,
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Review),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[get("/{reviewee_id}")]
    async fn get_reviews_by_reviewee_id_paginated(
        review_service: Data<ReviewServiceDependency>,
        Path(reviewee_id): Path<Ulid>,
        Validated(Query(pagination)): Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>> {
        let resp = review_service
            .find_all_by_reviewee(
                reviewee_id,
                pagination.into(),
                reviewee_id != user.id,
            )
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                ("reviewee_id" = Ulid, description = ""),
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            request_body(
                description = "",
                content = UpsertReview,
            ),
            responses(
                (status = 200, description = "", body = Review),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[put("/{reviewee_id}")]
    async fn upsert_review_by_id(
        review_service: Data<ReviewServiceDependency>,
        Path(reviewee_id): Path<Ulid>,
        user: ReqData<User>,
        Validated(Json(body)): Validated<Json<UpsertReview>>,
    ) -> Json<Review> {
        let resp = review_service
            .upsert_by_id(user.id, reviewee_id, body)
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                ("reviewee_id" = Ulid, description = ""),
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[delete("/{reviewee_id}")]
    async fn delete_review_by_id(
        review_service: Data<ReviewServiceDependency>,
        Path(reviewee_id): Path<Ulid>,
        user: ReqData<User>,
    ) -> HttpResponse {
        review_service
            .delete_by_id(user.id, reviewee_id, false, true)
            .await?;
        HttpResponse::NoContent().finish()
    }

    ///
    #[openapi(
            params(
                Pagination,
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[get("/my")]
    async fn get_current_reviews_received_paginated(
        review_service: Data<ReviewServiceDependency>,
        Validated(Query(pagination)): Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>> {
        let resp = review_service
            .find_all_by_reviewee(user.id, pagination.into(), false)
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                Pagination,
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[get("/my/sent")]
    async fn get_current_reviews_sent_paginated(
        review_service: Data<ReviewServiceDependency>,
        Validated(Query(pagination)): Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>> {
        let resp = review_service
            .find_all_by_reviewer(user.id, pagination.into(), false)
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                ("reviewer_id" = Ulid, description = ""),
                Pagination,
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[get("/{reviewer_id}/sent")]
    async fn get_reviews_by_reviewer_id_paginated(
        review_service: Data<ReviewServiceDependency>,
        Path(reviewer_id): Path<Ulid>,
        Validated(Query(pagination)): Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>> {
        let resp = review_service
            .find_all_by_reviewer(
                reviewer_id,
                pagination.into(),
                reviewer_id != user.id,
            )
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                ("reviewee_id" = Ulid, description = ""),
                ("reviewer_id" = Ulid, description = ""),
            ),
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizer" = []),
            ),
            responses(
                (status = 200, description = "", body = Review),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[get("/{reviewee_id}/{reviewer_id}")]
    async fn get_review_by_reviewee_id_and_reviewer_id(
        review_service: Data<ReviewServiceDependency>,
        Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        user: ReqData<User>,
    ) -> Json<Review> {
        let resp = review_service
            .get_by_id(
                reviewer_id,
                reviewee_id,
                reviewer_id != user.id,
                reviewee_id != user.id,
            )
            .await?;
        Json(resp)
    }

    ///
    #[openapi(
            params(
                ("reviewee_id" = Ulid, description = ""),
                ("reviewer_id" = Ulid, description = ""),
            ),
            security(
                ("organizer" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
    #[delete("/{reviewee_id}/{reviewer_id}")]
    async fn delete_review_by_reviewee_id_and_reviewer_id(
        review_service: Data<ReviewServiceDependency>,
        Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        user: ReqData<User>,
    ) -> HttpResponse {
        review_service
            .delete_by_id(
                reviewer_id,
                reviewee_id,
                reviewer_id != user.id,
                reviewee_id != user.id,
            )
            .await?;
        HttpResponse::NoContent().finish()
    }
}
