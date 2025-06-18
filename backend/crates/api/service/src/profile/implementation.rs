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
use ulid::Ulid;
use utils::LGTM;

use super::{
    PROFILES_BY_CITY_COUNT_METRIC_NAME, ProfileService, ProfileServiceResult,
};
use crate::{common::ServiceError, user::UserServiceDependency};

#[implementation(result = ProfileServiceResult)]
pub mod service {
    struct ProfileServiceImpl {
        user_repository: UserRepositoryDependency,
        profile_repository: ProfileRepositoryDependency,
        user_service: UserServiceDependency,
    }

    impl ProfileService for ProfileServiceImpl {
        async fn upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
            has_avatar: Option<bool>,
            check_user: bool,
        ) -> Profile {
            if check_user {
                self.user_service.get_by_id(id).await?;
            }

            let profile_id: ProfileId = id.into();
            let profile = self
                .profile_repository
                .find_by_id(profile_id.clone())
                .await?;
            let profile = self
                .profile_repository
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
                            profile.map(|p| p.has_avatar).unwrap_or(false),
                        ),
                    },
                )
                .await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(Some(profile_id.clone())),
                        ..Default::default()
                    },
                )
                .await?;
            profile.into()
        }

        async fn find_by_id(
            &self,
            id: Ulid,
            check_user: bool,
        ) -> Option<Profile> {
            if check_user {
                self.user_service.get_by_id(id).await?;
            }

            self.profile_repository
                .find_by_id(id.into())
                .await?
                .map(Profile::from)
        }

        async fn get_by_id(&self, id: Ulid, check_user: bool) -> Profile {
            self.find_by_id(id, check_user).await?.ok_or(
                ServiceError::NotFound("Profile with provided id".into()),
            )?
        }

        async fn delete_by_id(&self, id: Ulid, check_user: bool) -> () {
            self.get_by_id(id, check_user).await?;
            self.profile_repository.delete_by_id(id.into()).await?;
            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(None),
                        ..Default::default()
                    },
                )
                .await?;
        }

        async fn init_metrics(&self) {
            describe_gauge!(
                PROFILES_BY_CITY_COUNT_METRIC_NAME,
                "The city from the profile"
            );

            let profile_repository = self.profile_repository.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(profiles_by_city) =
                        profile_repository.count_by_city().await
                    {
                        profiles_by_city.iter().for_each(|(city, count)| {
                            gauge!(PROFILES_BY_CITY_COUNT_METRIC_NAME, "city" => city.to_string()).set(*count);
                        });
                    }

                    tokio::time::sleep(LGTM::METRIC_SCRAPE_INTERVAL).await;
                }
            });
        }
    }
}
