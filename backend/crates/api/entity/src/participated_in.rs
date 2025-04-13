use macros::entity;

use crate::{
    specialization::SpecializationId, technology::TechnologyId, tour::TourId,
    user::UserId,
};

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

impl CreateParticipatedIn {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> ParticipatedIn {
        ParticipatedIn {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
            score: self.score,
            specialization: self.specialization,
            technologies: self.technologies,
        }
    }
}
