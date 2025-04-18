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

impl From<CreateParticipatedIn> for ParticipatedIn {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_relation: CreateParticipatedIn) -> Self {
        Self {
            id: create_relation.get_id(),
            r#in: create_relation.r#in,
            out: create_relation.out,
            score: create_relation.score,
            specialization: create_relation.specialization,
            technologies: create_relation.technologies,
        }
    }
}
