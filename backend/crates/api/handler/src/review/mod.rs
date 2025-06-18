use actix_web::{
    HttpResponse,
    middleware::from_fn,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::{Path, Query};
use actix_web_validation::Validated;
use dto::{
    Pagination,
    review::{Review, UpsertReview},
    user::{User, UserRole},
};
use macros::handler;
use service::review::ReviewServiceDependency;
use ulid::Ulid;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub mod implementation;

use crate::common::{
    HandlerError, guard::UserRoleGuard, middleware::user_extractor_middleware,
};

#[handler(error = HandlerError)]
pub trait ReviewHandler {
    fn routes(
        review_service: ReviewServiceDependency,
        cfg: &mut ServiceConfig,
    ) {
        cfg.app_data(Data::new(review_service)).service(
            scope("/reviews")
                .wrap(from_fn(user_extractor_middleware))
                .service(Self::get_current_reviews_received_paginated())
                .service(Self::get_current_reviews_sent_paginated())
                .service(Self::get_reviews_by_reviewee_id_paginated())
                .service(Self::upsert_review_by_id())
                .service(Self::delete_review_by_id())
                .service(Self::get_reviews_by_reviewer_id_paginated())
                .service(Self::get_review_by_reviewee_id_and_reviewer_id())
                .service(
                    scope("")
                        .guard(UserRoleGuard::new(&[UserRole::Organizer]))
                        .service(
                            Self::delete_review_by_reviewee_id_and_reviewer_id(
                            ),
                        ),
                ),
        );
    }

    async fn upsert_review_by_id(
        review_service: Data<ReviewServiceDependency>,
        path: Path<Ulid>,
        user: ReqData<User>,
        body: Validated<Json<UpsertReview>>,
    ) -> Json<Review>;

    async fn delete_review_by_id(
        review_service: Data<ReviewServiceDependency>,
        path: Path<Ulid>,
        user: ReqData<User>,
    ) -> HttpResponse;

    async fn get_current_reviews_sent_paginated(
        review_service: Data<ReviewServiceDependency>,
        query: Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>>;

    async fn get_current_reviews_received_paginated(
        review_service: Data<ReviewServiceDependency>,
        query: Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>>;

    async fn get_reviews_by_reviewer_id_paginated(
        review_service: Data<ReviewServiceDependency>,
        path: Path<Ulid>,
        query: Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>>;

    async fn get_reviews_by_reviewee_id_paginated(
        review_service: Data<ReviewServiceDependency>,
        path: Path<Ulid>,
        query: Validated<Query<Pagination>>,
        user: ReqData<User>,
    ) -> Json<Vec<Review>>;

    async fn get_review_by_reviewee_id_and_reviewer_id(
        review_service: Data<ReviewServiceDependency>,
        path: Path<(Ulid, Ulid)>,
        user: ReqData<User>,
    ) -> Json<Review>;

    async fn delete_review_by_reviewee_id_and_reviewer_id(
        review_service: Data<ReviewServiceDependency>,
        path: Path<(Ulid, Ulid)>,
        user: ReqData<User>,
    ) -> HttpResponse;
}
