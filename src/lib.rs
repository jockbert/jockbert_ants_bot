use ants_ai_challenge_api::*;
use rand::Rng;

#[macro_use]
mod utilities;

mod ant_crash_filter;
mod avoid_water_filter;
mod world_step;

use crate::ant_crash_filter::AntCrashFilter;
use crate::avoid_water_filter::AvoidWaterFilter;
use crate::world_step::*;

#[derive(Default)]
pub struct FooAgent {
    params: GameParameters,
}

/// Generates a random direction.
fn random_direction(dirs: Vec<Direction>) -> Direction {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0 as usize, dirs.len());
    *dirs.get(index).expect("no out of bounds")
}

impl Agent for FooAgent {
    fn prepare(&mut self, params: GameParameters) {
        self.params = params;
    }

    fn make_turn(
        &mut self,
        world: WorldState,
        turn_count: u32,
    ) -> Orders {
        let my_ants = world.live_ants_for_player(0);
        let size =
            pos(self.params.rows as u16, self.params.cols as u16);

        use std::{thread, time};
        let delay = time::Duration::from_millis(50);
        thread::sleep(delay);
        eprintln!(
            "\nTurn {}\n{}\n",
            turn_count,
            utilities::serialize_world(&world, &size)
        );
        thread::sleep(delay);

        let world_step = &mut BasicWorldStep::new(world, size);

        let crash_filter = &mut AntCrashFilter::new(world_step);
        let mut water_filter = AvoidWaterFilter::new(crash_filter);

        let orders: Vec<Order> = my_ants
            .iter()
            .map(|ant| {
                ant.order(random_direction(
                    water_filter.available_directions(ant),
                ))
            })
            .collect();

        for order in orders {
            water_filter.add_order(order.clone());
        }

        water_filter.get_orders()
    }
}
