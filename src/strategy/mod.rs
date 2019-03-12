use crate::world_step::WorldStep;
use ants_ai_challenge_api::*;

pub mod random_walk;

pub trait Strategy {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants: Vec<Position>,
    ) -> (Vec<Position>, Orders);
}
