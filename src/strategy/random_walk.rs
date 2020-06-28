use crate::strategy::Strategy;
use crate::world_step::WorldStep;
use ants_ai_challenge_api::Direction;
use ants_ai_challenge_api::Orders;
use ants_ai_challenge_api::Position;
use rand::Rng;
use std::collections::HashSet;

pub struct RandomWalk {}

/// Generates a random direction.
fn random_direction(dirs: &[Direction]) -> Option<Direction> {
    if dirs.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0 as usize, dirs.len());
        let dir = *dirs.get(index).expect("no out of bounds");
        Some(dir)
    }
}

impl Strategy for RandomWalk {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants: &HashSet<Position>,
    ) -> Orders {
        ants.iter()
            .flat_map(|ant| {
                random_direction(
                    &world_step.available_directions(ant),
                )
                .map(|dir| ant.order(dir))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_step::avoid_water_filter::*;
    use std::iter::FromIterator;

    #[test]
    fn no_random_order_to_make() {
        let world_step = AvoidWaterFilter::new_from_line_map(
            "%%%
             %a%
             %%%",
        );

        let strategy = RandomWalk {};
        let ants = HashSet::from_iter(
            world_step.all_my_ants().iter().cloned(),
        );

        strategy.apply(&world_step, &ants);
    }
}
