use crate::strategy::*;
use crate::world_step::*;

use std::collections::HashSet;

pub struct HillRaiser {}

impl Strategy for HillRaiser {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants_available: &mut HashSet<Position>,
    ) -> Orders {
        best_orders_to_target(
            &world_step.get_positions(Tile::Hill(1)),
            world_step,
            ants_available,
            5,
            20,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avoid_own_hills() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "------
             -0-a--
             ------",
        );
        let strategy = &HillRaiser {};

        let ant = pos(1, 3);
        let mut ants = set![ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![ant]];

        assert_eq![actual_orders, vec![]];
    }

    #[test]
    fn go_to_nearest_hill() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -1-a--a-1-
             ----------",
        );
        let strategy = &HillRaiser {};

        let left_ant = pos(1, 3);
        let right_ant = pos(1, 6);
        let mut ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![]];

        assert_eq![
            actual_orders,
            vec![left_ant.west(), right_ant.east()]
        ];
    }

    #[test]
    fn orders_limited_by_hills_so_use_nearest_ant() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -a---1--a-
             ----------",
        );
        let strategy = &HillRaiser {};

        let left_ant = pos(1, 1);
        let right_ant = pos(1, 8);
        let mut ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![left_ant]];
        assert_eq![actual_orders, vec![right_ant.west()]];
    }

    #[test]
    fn orders_limited_by_ants_so_go_to_neares_hill() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----1----
             ---------
             1---a---1
             ---------
             ---------
             ----1----",
        );
        let strategy = &HillRaiser {};

        let ant = pos(2, 4);
        let mut ants = set![ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![]];
        assert_eq![actual_orders, vec![ant.north()]];
    }
}
