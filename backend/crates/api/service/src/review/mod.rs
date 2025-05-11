use dto::review::{Review, UpsertReview};
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    Review
        Err: ServiceError
    {
        upsert_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid, object: UpsertReview) -> Review;
        find_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> Option<Review>;
        get_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> Review;
        find_all_by_reviewer(&self, reviewer_id: Ulid, pagination: (u16, u64)) -> Vec<Review>;
        find_all_by_reviewee(&self, reviewee_id: Ulid, pagination: (u16, u64)) -> Vec<Review>;
        delete_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> ();
    }
}
