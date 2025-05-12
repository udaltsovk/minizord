use std::time::Duration;

use dto::profile::{Profile, UpsertProfile};
use entity::{
    profile::{ProfileId, UpsertProfile as UpsertProfileEntity},
    user::UserUpdate as UserEntityUpdate,
};
use macros::implementation;
use metrics::{describe_gauge, gauge};
use repository::{
    profile::ProfileRepositoryDependency, user::UserRepositoryDependency,
};
use tracing::instrument;
use ulid::Ulid;

use super::{
    PROFILES_BY_CITY_METRIC_NAME, PROFILES_FILLED_METRIC_NAME, ProfileService,
    ProfileServiceResult,
};
use crate::{common::ServiceError, user::UserServiceDependency};

implementation! {
    ProfileService {
        user_repository: UserRepositoryDependency,
        profile_repository: ProfileRepositoryDependency,
        user_service: UserServiceDependency,
    } as ProfileServiceImpl {
        #[instrument(skip_all, name = "ProfileService::update_by_id")]
        async fn upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
            has_avatar: Option<bool>,
            check_user: bool,
        ) -> Profile {
            if check_user {
                self.user_service
                    .get_by_id(id)
                    .await?;
            }

            let profile_id: ProfileId = id.into();
            let profile = self.profile_repository
                .find_by_id(profile_id.clone())
                .await?;
            let profile = self.profile_repository
                .upsert_by_id(
                    profile_id.clone(),
                    UpsertProfileEntity {
                        name: object.name,
                        surname: object.surname,
                        telegram: object.telegram,
                        city: object.city,
                        bio: object.bio,
                        portfolio_urls: object.portfolio_urls,
                        has_avatar: has_avatar.unwrap_or(
                            profile.map(|p| p.has_avatar).unwrap_or(false)
                        ),
                    }
                )
                .await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(Some(profile_id.clone())),
                        ..Default::default()
                    }
                )
                .await?;
            profile.into()
        }

        #[instrument(skip_all, name = "ProfileService::find_by_id")]
        async fn find_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> Option<Profile> {
            if check_user {
                self.user_service
                    .get_by_id(id)
                    .await?;
            }

            self.profile_repository
                .find_by_id(id.into())
                .await?
                .map(Profile::from)
        }

        #[instrument(skip_all, name = "ProfileService::get_by_id")]
        async fn get_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> Profile {
            self
                .find_by_id(id, check_user)
                .await?
                .ok_or(
                    ServiceError::NotFound("Profile with provided id".into())
                )?
        }

        #[instrument(skip_all, name = "ProfileService::delete_by_id")]
        async fn delete_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> () {
            self.get_by_id(id, check_user).await?;
            self.profile_repository
                .delete_by_id(id.into())
                .await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(None),
                        ..Default::default()
                    }
                )
                .await?;
        }

        #[instrument(skip_all, name = "ProfileService::init_metrics")]
        async fn init_metrics(&self) {
            describe_gauge!(PROFILES_FILLED_METRIC_NAME, "The number of filled profiles");
            describe_gauge!(PROFILES_BY_CITY_METRIC_NAME, "The city from the profile");

            let profile_repository = self.profile_repository.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(filled_profiles) = profile_repository
                        .count_filled()
                        .await
                    {
                        gauge!(PROFILES_FILLED_METRIC_NAME).set(filled_profiles);
                    }
                    if let Ok(profiles_by_city) = profile_repository
                        .count_by_city()
                        .await
                    {
                        profiles_by_city.into_iter().for_each(|(city, count)| {
                            gauge!(PROFILES_BY_CITY_METRIC_NAME, "city" => city).set(count);
                        });
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });
        }
    }
}
