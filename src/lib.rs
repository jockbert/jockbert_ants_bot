use ants_ai_challenge_api::*;
use rand::Rng;

pub struct FooAgent {}

/// Generates a random direction.
fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();
    let directions = [North, South, East, West];
    let index = rng.gen_range(0 as usize, directions.len());
    *directions.get(index).expect("no out of bounds")
}

/// Generates a random order, given a position.
fn random_order(pos: &Position) -> Order {
    (pos.clone(), random_direction())
}

impl Agent for FooAgent {
    fn prepare(&mut self, _params: GameParameters) {
        // do nothing in prep
    }

    fn make_turn(&mut self, world: WorldState, _turn_count: u32) -> Orders {
        world
            .live_ants_for_player(0)
            .iter()
            .map(random_order)
            .collect()
    }
}
