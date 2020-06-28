use crate::strategy::*;
use crate::world_step::*;

use std::collections::HashSet;

pub struct GatherFood {}

impl Strategy for GatherFood {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants_available: &HashSet<Position>,
    ) -> Orders {
        best_orders_to_target(
            &world_step.get_positions(Tile::Food),
            world_step,
            ants_available,
            3,
            15,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_to_nearest_food() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -*-a--a-*-
             ----------",
        );
        let strategy = &GatherFood {};

        let left_ant = pos(1, 3);
        let right_ant = pos(1, 6);
        let ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &ants);

        assert_eq![
            actual_orders,
            vec![left_ant.west(), right_ant.east()]
        ];
    }

    #[test]
    fn orders_limited_by_food_so_use_nearest_ant() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -a---*--a-
             ----------",
        );
        let strategy = &GatherFood {};

        let left_ant = pos(1, 1);
        let right_ant = pos(1, 8);
        let ants = set![left_ant, right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &ants);
        assert_eq![actual_orders, vec![right_ant.west()]];
    }

    #[test]
    fn orders_limited_by_ants_so_go_to_neares_food() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----*----
             ---------
             *---a---*
             ---------
             ---------
             ----*----",
        );
        let strategy = &GatherFood {};

        let ant = pos(2, 4);
        let ants = set![ant.clone()];

        let actual_orders = strategy.apply(world_step, &ants);
        assert_eq![actual_orders, vec![ant.north()]];
    }
}
