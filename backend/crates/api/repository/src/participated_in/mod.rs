use macros::{crud_repository, entity};

use crate::{
    specialization::SpecializationId, technology::TechnologyId, tour::TourId,
    user::UserId,
};

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    UserId -> ParticipatedIn -> TourId {
        fields {
            score: u16,
            specialization: SpecializationId,
            technologies: Vec<TechnologyId>,
        },
        create {
            score: u16,
            specialization: SpecializationId,
            technologies: Vec<TechnologyId>,
        },
        update {
            score: u16,
            specialization: SpecializationId,
            technologies: Vec<TechnologyId>,
        }
    }
}

crud_repository! {
    UserId -> ParticipatedIn -> TourId
}

impl CreateParticipatedIn {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> ParticipatedIn {
        ParticipatedIn {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            score: self.score,
            specialization: self.specialization,
            technologies: self.technologies,
        }
    }
}
