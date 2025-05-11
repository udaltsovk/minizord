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

handler! {
    Review
        Err: HandlerError,
        Impl: ImplementedReviewHandler
    {
        #routes(review_service: ReviewServiceDependency) {
            move |cfg: &mut ServiceConfig| {
                cfg.app_data(Data::new(review_service))
                    .service(scope("/reviews")
                        .wrap(from_fn(user_extractor_middleware))
                        .service(Self::get_reviews_by_reviewee_id_paginated())
                        .service(Self::upsert_review_by_id())
                        .service(Self::delete_review_by_id())
                        .service(Self::get_current_reviews_received_paginated())
                        .service(Self::get_current_reviews_sent_paginated())
                        .service(Self::get_reviews_by_reviewer_id_paginated())
                        .service(Self::get_review_by_reviewee_id_and_reviewer_id())
                        .service(scope("")
                            .guard(UserRoleGuard::new(&[UserRole::Organizer]))
                            .service(Self::delete_review_by_reviewee_id_and_reviewer_id())
                        )
                    );
            }
        }

        upsert_review_by_id(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            path: Path<Ulid>,
            body: Validated<Json<UpsertReview>>,
        ) -> Json<Review>;

        delete_review_by_id(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            path: Path<Ulid>,
        ) -> HttpResponse;

        get_current_reviews_sent_paginated(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            query: Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>>;

        get_current_reviews_received_paginated(
            review_service: Data<ReviewServiceDependency>,
            user: ReqData<User>,
            query: Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>>;

        get_reviews_by_reviewer_id_paginated(
            review_service: Data<ReviewServiceDependency>,
            path: Path<Ulid>,
            query: Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>>;

        get_reviews_by_reviewee_id_paginated(
            review_service: Data<ReviewServiceDependency>,
            path: Path<Ulid>,
            query: Validated<Query<Pagination>>,
        ) -> Json<Vec<Review>>;

        get_review_by_reviewee_id_and_reviewer_id(
            review_service: Data<ReviewServiceDependency>,
            path: Path<(Ulid, Ulid)>,
        ) -> Json<Review>;

        delete_review_by_reviewee_id_and_reviewer_id(
            review_service: Data<ReviewServiceDependency>,
            path: Path<(Ulid, Ulid)>,
        ) -> HttpResponse;
    }
}
