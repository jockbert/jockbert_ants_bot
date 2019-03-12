use crate::strategy::Strategy;
use crate::world_step::WorldStep;
use ants_ai_challenge_api::Direction;
use ants_ai_challenge_api::Order;
use ants_ai_challenge_api::Orders;
use ants_ai_challenge_api::Position;

use rand::Rng;

pub struct RandomWalk {}

/// Generates a random direction.
fn random_direction(dirs: Vec<Direction>) -> Direction {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0 as usize, dirs.len());
    *dirs.get(index).expect("no out of bounds")
}

impl Strategy for RandomWalk {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants: Vec<Position>,
    ) -> (Vec<Position>, Orders) {
        let orders: Vec<Order> = ants
            .iter()
            .map(|ant| {
                ant.order(random_direction(
                    world_step.available_directions(ant),
                ))
            })
            .collect();

        (vec![], orders)
    }
}
