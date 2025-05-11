use dto::review::{Review, UpsertReview};
use entity::reviewed::UpsertReviewed;
use macros::implementation;
use repository::reviewed::ReviewedRepositoryDependency;
use ulid::Ulid;

use super::{ReviewService, ReviewServiceResult};
use crate::{common::ServiceError, user::UserServiceDependency};

implementation! {
    ReviewService {
        reviewed_repository: ReviewedRepositoryDependency,
        user_service: UserServiceDependency,
    } as ReviewServiceImpl {
        upsert_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid, object: UpsertReview) -> Review {
            self.user_service
                .get_by_id(reviewer_id)
                .await?;
            self.user_service
                .get_by_id(reviewee_id)
                .await?;

            if reviewer_id == reviewee_id {
                Err(ServiceError::BadRequest("You can't review yourself".into()))?;
            }

            self.reviewed_repository
                .upsert_by_in_and_out(
                    reviewer_id.into(),
                    reviewee_id.into(),
                    UpsertReviewed {
                        r#in: reviewer_id.into(),
                        out: reviewee_id.into(),
                        score: object.score,
                        review: object.review,
                    }
                )
                .await?
                .into()
        }

        find_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> Option<Review> {
            self.user_service
                .get_by_id(reviewer_id)
                .await?;
            self.user_service
                .get_by_id(reviewee_id)
                .await?;

            self.reviewed_repository
                .find_by_in_and_out(reviewer_id.into(), reviewee_id.into())
                .await?
                .map(Review::from)
        }
        get_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> Review {
            self
                .find_by_id(reviewer_id, reviewee_id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Review with provided id".into())
                )?
        }

        find_all_by_reviewer(&self, reviewer_id: Ulid, (limit, offset): (u16, u64)) -> Vec<Review> {
            self.user_service
                .get_by_id(reviewer_id)
                .await?;

            self.reviewed_repository
                .find_all_by_in(reviewer_id.into(), limit, offset)
                .await?
                .into_iter()
                .map(Review::from)
                .collect()
        }

        find_all_by_reviewee(&self, reviewee_id: Ulid, (limit, offset): (u16, u64)) -> Vec<Review> {
            self.user_service
                .get_by_id(reviewee_id)
                .await?;

            self.reviewed_repository
                .find_all_by_out(reviewee_id.into(), limit, offset)
                .await?
                .into_iter()
                .map(Review::from)
                .collect()
        }

        delete_by_id(&self, reviewer_id: Ulid, reviewee_id: Ulid) -> () {
            self.get_by_id(reviewer_id, reviewee_id).await?;
            self.reviewed_repository
                .delete_by_in_and_out(reviewer_id.into(), reviewee_id.into())
                .await?;
        }
    }
}
