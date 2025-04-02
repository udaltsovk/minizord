use dto::profile::{Profile, UpsertProfile};
use macros::implementation;
use repository::{
    profile::{
        ProfileRepositoryDependency, UpsertProfile as UpsertProfileEntity,
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
        upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
        ) -> Profile {
            self.profile_repository
                .upsert_by_id(
                    id.into(),
                    UpsertProfileEntity {
                        name: object.name,
                        surname: object.surname,
                        city: object.city,
                        bio: object.bio,
                        portfolio_urls: object.portfolio_urls.clone(),
                    }
                )
                .await?
                .into()
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
