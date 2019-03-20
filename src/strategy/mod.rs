use crate::world_step::WorldStep;
use ants_ai_challenge_api::*;

pub mod composite_strategy;
pub mod gather_food;
pub mod random_walk;
pub mod search;

pub use composite_strategy::*;
pub use gather_food::*;
pub use random_walk::*;

pub trait Strategy {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants_available: &Vec<Position>,
    ) -> (Vec<Position>, Orders);
}
