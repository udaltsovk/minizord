use dto::review::{Review, UpsertReview};
use macros::{metric_name, service};
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

#[service(error = ServiceError)]
pub trait ReviewService {
    async fn upsert_by_id(
        &self,
        reviewer_id: Ulid,
        reviewee_id: Ulid,
        object: UpsertReview,
    ) -> Review;

    async fn find_by_id(
        &self,
        reviewer_id: Ulid,
        reviewee_id: Ulid,
        check_reviewer: bool,
        check_reviewee: bool,
    ) -> Option<Review>;

    async fn get_by_id(
        &self,
        reviewer_id: Ulid,
        reviewee_id: Ulid,
        check_reviewer: bool,
        check_reviewee: bool,
    ) -> Review;

    async fn find_all_by_reviewer(
        &self,
        reviewer_id: Ulid,
        pagination: (u16, u64),
        check_reviewer: bool,
    ) -> Vec<Review>;

    async fn find_all_by_reviewee(
        &self,
        reviewee_id: Ulid,
        pagination: (u16, u64),
        check_reviewee: bool,
    ) -> Vec<Review>;

    async fn delete_by_id(
        &self,
        reviewer_id: Ulid,
        reviewee_id: Ulid,
        check_reviewer: bool,
        check_reviewee: bool,
    ) -> ();
    async fn init_metrics(&self);
}

metric_name!(REVIEWS_BY_SCORE_COUNT, "reviews_by_score_count");
metric_name!(REVIEWS_BY_SCORE_SUM, "reviews_by_score_sum");
