use crate::strategy::Strategy;
use ants_ai_challenge_api::*;
use std::collections::HashSet;

#[macro_use]
#[cfg(test)]
mod utilities;
mod strategy;
mod world_step;

use crate::strategy::random_walk::RandomWalk;
use crate::world_step::ant_crash_filter::AntCrashFilter;
use crate::world_step::avoid_water_filter::AvoidWaterFilter;
use crate::world_step::basic_world_step::BasicWorldStep;
use crate::world_step::WorldStep;

#[derive(Default)]
pub struct FooAgent {
    params: GameParameters,
    accumulated_water: HashSet<Position>,
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
        world.waters.iter().cloned().for_each(|w| {
            self.accumulated_water.insert(w);
        });

        let world = WorldState {
            dead_ants: world.dead_ants.clone(),
            foods: world.foods.clone(),
            hills: world.hills.clone(),
            live_ants: world.live_ants.clone(),
            waters: self.accumulated_water.iter().cloned().collect(), // world.copy()
        };

        let my_ants = world.live_ants_for_player(0);
        let size =
            pos(self.params.rows as u16, self.params.cols as u16);

        // eprintln!(
        //    "\nTurn {}\n{}\n",
        //    turn_count,
        //    utilities::serialize_world(&world, &size)
        // );

        let world_step = &mut BasicWorldStep::new(world, size);

        let crash_filter = &mut AntCrashFilter::new(world_step);
        let mut water_filter = AvoidWaterFilter::new(crash_filter);

        let strategy = &RandomWalk {};

        let (_ants_left, orders) =
            strategy.apply(&water_filter, my_ants);

        for order in orders {
            water_filter.add_order(order.clone());
        }

        water_filter.get_orders()
    }
}
