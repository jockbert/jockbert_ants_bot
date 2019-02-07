use ants_ai_challenge_api::*;
use rand::Rng;

mod ant_crash_filter;

#[cfg(test)]
mod utilities;

use crate::ant_crash_filter::AntCrashFilter;

pub struct FooAgent {
    params: GameParameters,
}

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

impl FooAgent {
    pub fn new() -> FooAgent {
        FooAgent {
            params: GameParameters {
                loadtime_ms: 0,
                turntime_ms: 0,
                rows: 0,
                cols: 0,
                turns: 0,
                viewradius2: 0,
                attackradius2: 0,
                spawnradius2: 0,
                player_seed: 0,
            },
        }
    }
}

impl Agent for FooAgent {
    fn prepare(&mut self, params: GameParameters) {
        self.params = params;
    }

    fn make_turn(
        &mut self,
        world: WorldState,
        _turn_count: u32,
    ) -> Orders {
        let my_ants = world.live_ants_for_player(0);

        let mut crash_filter = AntCrashFilter::new(
            world,
            pos(self.params.rows as u16, self.params.cols as u16),
        );

        let orders: Vec<Order> =
            my_ants.iter().map(random_order).collect();

        for order in orders {
            crash_filter = crash_filter.add(order.clone());
        }

        crash_filter.get_orders()
    }
}
