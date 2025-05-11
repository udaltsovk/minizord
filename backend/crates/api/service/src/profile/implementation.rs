use dto::profile::{Profile, UpsertProfile};
use entity::{
    profile::{ProfileId, UpsertProfile as UpsertProfileEntity},
    user::UserUpdate as UserEntityUpdate,
};
use macros::implementation;
use repository::{
    profile::ProfileRepositoryDependency, user::UserRepositoryDependency,
};
use tracing::instrument;
use ulid::Ulid;

use super::{ProfileService, ProfileServiceResult};
use crate::common::ServiceError;

implementation! {
    ProfileService {
        user_repository: UserRepositoryDependency,
        profile_repository: ProfileRepositoryDependency,
    } as ProfileServiceImpl {
        #[instrument(skip_all, name = "ProfileService::update_by_id")]
        async fn upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
            has_avatar: Option<bool>,
        ) -> Profile {
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
        ) -> Option<Profile> {
            self.profile_repository
                .find_by_id(id.into())
                .await?
                .map(Profile::from)
        }

        #[instrument(skip_all, name = "ProfileService::get_by_id")]
        async fn get_by_id(
            &self,
            id: Ulid,
        ) -> Profile {
            self
                .find_by_id(id)
                .await?
                .ok_or(
                    ServiceError::NotFound("Profile with provided id".into())
                )?
        }

        #[instrument(skip_all, name = "ProfileService::delete_by_id")]
        async fn delete_by_id(
            &self,
            id: Ulid,
        ) -> () {
            self.get_by_id(id).await?;
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
    }
}
