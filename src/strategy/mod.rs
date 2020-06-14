use crate::strategy::multisearch::*;
use crate::world_step::WorldStep;
use ants_ai_challenge_api::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub mod combat;
pub mod composite_strategy;
pub mod gather_food;
pub mod hill_raiser;
pub mod multisearch;
pub mod random_walk;
pub mod search;
pub mod spread_out_scout;

pub use composite_strategy::*;
pub use gather_food::*;
pub use hill_raiser::*;
pub use random_walk::*;
pub use spread_out_scout::*;

pub trait Strategy {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants_available: &mut HashSet<Position>,
    ) -> Orders;
}

fn best_orders_to_target(
    targets: &[Position],
    world_step: &dyn WorldStep,
    ants_available: &mut HashSet<Position>,
    max_result_len: usize,
    cutoff_len: usize,
) -> Orders {
    let target_set = HashSet::from_iter(targets.iter().cloned());

    let results = create_multisearch().search_all(
        world_step,
        ants_available,
        &target_set,
        max_result_len,
        cutoff_len,
    );

    for result in &results {
        ants_available.remove(&result.first_step());
    }

    results
        .iter()
        .flat_map(|r| r.first_order(world_step.size()))
        .collect()
}
