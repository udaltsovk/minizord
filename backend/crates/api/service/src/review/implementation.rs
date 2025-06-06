use dto::review::{Review, UpsertReview};
use entity::reviewed::UpsertReviewed;
use macros::implementation;
use metrics::{describe_gauge, gauge};
use repository::reviewed::ReviewedRepositoryDependency;
use tracing::instrument;
use ulid::Ulid;
use utils::LGTM;

use super::{
    REVIEWS_BY_SCORE_COUNT_METRIC_NAME, REVIEWS_BY_SCORE_SUM_METRIC_NAME,
    ReviewService, ReviewServiceResult,
};
use crate::{common::ServiceError, user::UserServiceDependency};

implementation! {
    ReviewService {
        reviewed_repository: ReviewedRepositoryDependency,
        user_service: UserServiceDependency,
    } as ReviewServiceImpl {
        #[instrument(skip_all, name = "ReviewService::upsert_by_id")]
        async fn upsert_by_id(
            &self,
            reviewer_id: Ulid,
            reviewee_id: Ulid,
            object: UpsertReview,
        ) -> Review {
            if reviewer_id == reviewee_id {
                Err(ServiceError::BadRequest("You can't review yourself".into()))?;
            }

            self.user_service
                .get_by_id(reviewee_id)
                .await?;

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

        #[instrument(skip_all, name = "ReviewService::find_by_id")]
        async fn find_by_id(
            &self,
            reviewer_id: Ulid,
            reviewee_id: Ulid,
            check_reviewer: bool,
            check_reviewee: bool,
        ) -> Option<Review> {
            if check_reviewer {
                self.user_service
                    .get_by_id(reviewer_id)
                    .await?;
            }
            if check_reviewee {
                self.user_service
                    .get_by_id(reviewee_id)
                    .await?;
            }

            self.reviewed_repository
                .find_by_in_and_out(reviewer_id.into(), reviewee_id.into())
                .await?
                .map(Review::from)
        }

        #[instrument(skip_all, name = "ReviewService::get_by_id")]
        async fn get_by_id(
            &self,
            reviewer_id: Ulid,
            reviewee_id: Ulid,
            check_reviewer: bool,
            check_reviewee: bool,
        ) -> Review {
            self
                .find_by_id(
                    reviewer_id,
                    reviewee_id,
                    check_reviewer,
                    check_reviewee,
                )
                .await?
                .ok_or(
                    ServiceError::NotFound("Review with provided id".into())
                )?
        }

        #[instrument(skip_all, name = "ReviewService::find_all_by_reviewer")]
        async fn find_all_by_reviewer(
            &self,
            reviewer_id: Ulid,
            (limit, offset): (u16, u64),
            check_reviewer: bool,
        ) -> Vec<Review> {
            if check_reviewer {
                self.user_service
                    .get_by_id(reviewer_id)
                    .await?;
            }

            self.reviewed_repository
                .find_all_by_in(reviewer_id.into(), limit, offset)
                .await?
                .into_iter()
                .map(Review::from)
                .collect()
        }

        #[instrument(skip_all, name = "ReviewService::find_all_by_reviewee")]
        async fn find_all_by_reviewee(
            &self,
            reviewee_id: Ulid,
            (limit, offset): (u16, u64),
            check_reviewee: bool,
        ) -> Vec<Review> {
            if check_reviewee {
                self.user_service
                    .get_by_id(reviewee_id)
                    .await?;
            }

            self.reviewed_repository
                .find_all_by_out(reviewee_id.into(), limit, offset)
                .await?
                .into_iter()
                .map(Review::from)
                .collect()
        }

        #[instrument(skip_all, name = "ReviewService::delete_by_id")]
        async fn delete_by_id(
            &self,
            reviewer_id: Ulid,
            reviewee_id: Ulid,
            check_reviewer: bool,
            check_reviewee: bool,
        ) -> () {
            self.get_by_id(
                reviewer_id,
                reviewee_id,
                check_reviewer,
                check_reviewee,
            ).await?;

            self.reviewed_repository
                .delete_by_in_and_out(reviewer_id.into(), reviewee_id.into())
                .await?;
        }

        #[instrument(skip_all, name = "ReviewService::init_metrics")]
        async fn init_metrics(&self) {
            describe_gauge!(REVIEWS_BY_SCORE_COUNT_METRIC_NAME, "The number of reviews by score");
            describe_gauge!(REVIEWS_BY_SCORE_SUM_METRIC_NAME, "The sum of review scores by score");

            let reviewed_repository = self.reviewed_repository.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(reviews_by_score) = reviewed_repository
                        .count_by_score()
                        .await
                    {
                        reviews_by_score.iter().for_each(|(score, count)| {
                            gauge!(REVIEWS_BY_SCORE_COUNT_METRIC_NAME, "score" => score.to_string()).set(*count);
                        });
                        reviews_by_score.iter().for_each(|(score, count)| {
                            gauge!(REVIEWS_BY_SCORE_SUM_METRIC_NAME, "score" => score.to_string()).set(count.saturating_mul((*score).into()))
                        });
                    }

                    tokio::time::sleep(LGTM::METRIC_SCRAPE_INTERVAL).await;
                }
            });
        }
    }
}
