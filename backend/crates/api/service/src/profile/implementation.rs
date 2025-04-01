use dto::profile::{CreateProfile, Profile, ProfileUpdate};
use macros::implementation;
use repository::{
    profile::{
        CreateProfile as CreateProfileEntity, ProfileRepositoryDependency,
        ProfileUpdate as ProfileEntityUpdate,
    },
    user::{UserRepositoryDependency, UserUpdate as UserEntityUpdate},
};
use ulid::Ulid;

use crate::common::ServiceError;

implementation! {
    ProfileService {
        user_repository: UserRepositoryDependency,
        profile_repository: ProfileRepositoryDependency,
    } as Implemented {
        create(
            &self,
            id: Ulid,
            new: CreateProfile,
        ) -> Profile {
            if self.profile_repository.exists_by_id(id.into()).await? {
                Err(ServiceError::AlreadyExists("Profile for provided user".into()))?
            }

            let profile = self.profile_repository
                .save(
                    CreateProfileEntity {
                        user: id.into(),
                        has_avatar: false,
                        name: new.name,
                        surname: new.surname,
                        city: new.city,
                        bio: new.bio,
                        portfolio_urls: new.portfolio_urls,
                    }
                ).await?;

            self.user_repository
                .update_by_id(
                    id.into(),
                    UserEntityUpdate {
                        profile: Some(Some(id.into())),
                        ..Default::default()
                    }
                )
                .await?;

            profile.into()
        }

        find_by_id(
            &self,
            id: Ulid,
        ) -> Option<Profile> {
            self.profile_repository
                .find_by_id(id.into())
                .await?
                .map(Profile::from)
        }

        get_by_id(
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

        update_by_id(
            &self,
            id: Ulid,
            update: ProfileUpdate,
        ) -> Profile {
            self.get_by_id(id).await?;
            self.profile_repository
                .update_by_id(
                    id.into(),
                    ProfileEntityUpdate {
                        name: update.name,
                        surname: update.surname,
                        city: update.city,
                        bio: update.bio,
                        portfolio_urls: update.portfolio_urls.clone(),
                    }
                )
                .await?
                .unwrap()
                .into()
        }

        delete_by_id(
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
