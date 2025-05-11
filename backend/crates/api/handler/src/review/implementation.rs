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
use macros::handler_implementation;
use service::review::ReviewServiceDependency;
use tracing::instrument;
use ulid::Ulid;

use super::{ReviewHandler, ReviewHandlerHelper, ReviewHandlerResult};
use crate::common::{ApiError, ValidationError, openapi};

handler_implementation! {
    ReviewHandler as ReviewHandlerImpl {
        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::get_reviews_by_reviewee_id_paginated")]
        async fn get_reviews_by_reviewee_id_paginated(
            review_service: Data<ReviewServiceDependency>,
            Path(reviewee_id): Path<Ulid>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = review_service
                .find_all_by_reviewee(reviewee_id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::upsert_review_by_id")]
        async fn upsert_review_by_id(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            Path(reviewee_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<UpsertReview>>,
        ) -> Json<Review> {
            let resp = review_service
                .upsert_by_id(user.id, reviewee_id, body)
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::delete_review_by_id")]
        async fn delete_review_by_id(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            Path(reviewee_id): Path<Ulid>,
        ) -> HttpResponse {
            review_service
                .delete_by_id(user.id, reviewee_id)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::get_current_reviews_received_paginated")]
        async fn get_current_reviews_received_paginated(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = review_service
                .find_all_by_reviewee(user.id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::get_current_reviews_sent_paginated")]
        async fn get_current_reviews_sent_paginated(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = review_service
                .find_all_by_reviewer(user.id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::get_reviews_by_reviewer_id_paginated")]
        async fn get_reviews_by_reviewer_id_paginated(
            review_service: Data<ReviewServiceDependency>,
            Path(reviewer_id): Path<Ulid>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = review_service
                .find_all_by_reviewer(reviewer_id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::get_review_by_reviewee_id_and_reviewer_id")]
        async fn get_review_by_reviewee_id_and_reviewer_id(
            review_service: Data<ReviewServiceDependency>,
            Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        ) -> Json<Review> {
            let resp = review_service
                .get_by_id(reviewer_id, reviewee_id)
                .await?;
            Json(resp)
        }

        ///
        ///
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
        #[instrument(skip_all, name = "ReviewHandler::delete_review_by_reviewee_id_and_reviewer_id")]
        async fn delete_review_by_reviewee_id_and_reviewer_id(
            review_service: Data<ReviewServiceDependency>,
            Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        ) -> HttpResponse {
            review_service
                .delete_by_id(reviewer_id, reviewee_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
