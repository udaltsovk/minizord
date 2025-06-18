use macros::relation;

use crate::{
    EntityId, specialization::SpecializationId, technology::TechnologyId,
    tour::TourId, user::UserId,
};

#[relation]
pub struct ParticipatedIn {
    pub r#in: UserId,
    pub out: TourId,

    #[field]
    #[create]
    #[update]
    pub score: u16,

    #[field]
    #[create]
    #[update]
    pub specialization: SpecializationId,

    #[field]
    #[create]
    #[update]
    pub technologies: Vec<TechnologyId>,
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
