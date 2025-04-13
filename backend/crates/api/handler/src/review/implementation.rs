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
use service::reviewed::ReviewedServiceDependency;
use ulid::Ulid;

use super::{ReviewHandler, ReviewHandlerHelper, ReviewHandlerResult};
use crate::common::{ApiError, ValidationError, openapi};

handler_implementation! {
    ReviewHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "upsert_review_by_id",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("reviewee_id" = Ulid, description = ""),
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
        upsert_by_id(
            reviewed_service: Data<ReviewedServiceDependency>,
            user: ReqData<User>,
            Path(reviewee_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<UpsertReview>>,
        ) -> Json<Review> {
            let resp = reviewed_service
                .upsert_by_id(user.id, reviewee_id, body)
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_review_by_id",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("reviewee_id" = Ulid, description = ""),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/{reviewee_id}")]
        delete_by_id(
            reviewed_service: Data<ReviewedServiceDependency>,
            user: ReqData<User>,
            Path(reviewee_id): Path<Ulid>,
        ) -> HttpResponse {
            reviewed_service
                .delete_by_id(user.id, reviewee_id)
                .await?;
            HttpResponse::NoContent().finish()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_reviews_sent_paginated",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                Pagination,
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/my")]
        get_current_sent_paginated(
            reviewed_service: Data<ReviewedServiceDependency>,
            user: ReqData<User>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = reviewed_service
                .find_all_by_reviewer(user.id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_reviews_received_paginated",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                Pagination,
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/my/received")]
        get_current_received_paginated(
            reviewed_service: Data<ReviewedServiceDependency>,
            user: ReqData<User>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = reviewed_service
                .find_all_by_reviewee(user.id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_reviews_by_reviewer_id_paginated",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("reviewer_id" = Ulid, description = ""),
                Pagination,
            ),
            responses(
                (status = 200, description = "", body = Vec<Review>),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{reviewer_id}/sent")]
        get_by_reviewer_id_paginated(
            reviewed_service: Data<ReviewedServiceDependency>,
            Path(reviewer_id): Path<Ulid>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = reviewed_service
                .find_all_by_reviewer(reviewer_id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_reviews_by_reviewee_id_paginated",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("reviewee_id" = Ulid, description = ""),
                Pagination,
            ),
            responses(
                (status = 200, description = "", body = Review),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{reviewee_id}")]
        get_by_reviewee_id_paginated(
            reviewed_service: Data<ReviewedServiceDependency>,
            Path(reviewee_id): Path<Ulid>,
            Validated(Query(pagination)): Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>> {
            let resp = reviewed_service
                .find_all_by_reviewee(reviewee_id, pagination.into())
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_review_by_reviewee_id_and_reviewer_id",
            security(
                ("participant" = []),
                ("mentor" = []),
                ("organizator" = []),
            ),
            params(
                ("reviewee_id" = Ulid, description = ""),
                ("reviewer_id" = Ulid, description = ""),
            ),
            responses(
                (status = 200, description = "", body = Review),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{reviewee_id}/{reviewer_id}")]
        get_by_reviewee_id_and_reviewer_id(
            reviewed_service: Data<ReviewedServiceDependency>,
            Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        ) -> Json<Review> {
            let resp = reviewed_service
                .get_by_id(reviewee_id, reviewer_id)
                .await?;
            Json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_review_by_reviewee_id_and_reviewer_id",
            security(
                ("organizator" = []),
            ),
            params(
                ("reviewee_id" = Ulid, description = ""),
                ("reviewer_id" = Ulid, description = ""),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/{reviewee_id}/{reviewer_id}")]
        delete_by_reviewee_id_and_reviewer_id(
            reviewed_service: Data<ReviewedServiceDependency>,
            Path((reviewee_id, reviewer_id)): Path<(Ulid, Ulid)>,
        ) -> HttpResponse {
            reviewed_service
                .delete_by_id(reviewee_id, reviewer_id)
                .await?;
            HttpResponse::NoContent().finish()
        }
    }
}
