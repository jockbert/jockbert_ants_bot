use ants_ai_challenge_api::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_use]
pub mod utilities;
pub mod strategy;
pub mod world_step;

use crate::strategy::*;
use crate::world_step::*;

#[derive(Default)]
pub struct FooAgent {
    params: GameParameters,
    accumulated_water: HashSet<Position>,
}

fn eprint(step: &impl WorldStep) {
    let ten_millis = std::time::Duration::from_millis(10);
    std::thread::sleep(ten_millis);
    eprintln!("");
    eprint!("{}", step.format("    ", false));
    eprintln!("");
    std::thread::sleep(ten_millis);
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
            live_ants: world.live_ants,
            waters: Vec::from_iter(
                self.accumulated_water.iter().cloned(),
            ),
        };

        let mut my_ants: HashSet<Position> = HashSet::from_iter(
            world.live_ants_for_player(0).iter().cloned(),
        );

        let size =
            pos(self.params.rows as u16, self.params.cols as u16);

        let world_step = BasicWorldStep::new(world, size);
        let crash_filter = AntCrashFilter::new(Box::new(world_step));
        let mut water_filter =
            AvoidWaterFilter::new(Box::new(crash_filter));

        let strategy = &CompositeStrategy::new_with_default();

        let orders = strategy.apply(&water_filter, &mut my_ants);

        for order in orders {
            water_filter.add_order(order.clone());
        }

        eprint(&water_filter);

        water_filter.get_orders()
    }
}
