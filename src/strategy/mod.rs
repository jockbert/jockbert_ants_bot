use crate::world_step::WorldStep;
use ants_ai_challenge_api::*;
use std::collections::HashSet;

pub mod composite_strategy;
pub mod gather_food;
pub mod random_walk;
pub mod spread_out_scout;
pub mod search;

pub use composite_strategy::*;
pub use gather_food::*;
pub use random_walk::*;
pub use spread_out_scout::*;

pub trait Strategy {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants_available: &mut HashSet<Position>,
    ) -> Orders;
}
