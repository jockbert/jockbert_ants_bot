use crate::strategy::search::*;
use crate::strategy::*;
use crate::world_step::*;
use ants_ai_challenge_api::Orders;
use ants_ai_challenge_api::Position;

pub struct GatherFood {}

impl Strategy for GatherFood {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants_available: &mut HashSet<Position>,
    ) -> Orders {
        let mut result_orders: Vec<Order> = Vec::new();

        for food in world_step.get_positions(Tile::Food) {
            // only interrested in the nearest ant
            let search_orders = create_search().search(
                world_step,
                food,
                ants_available,
                1,
                20,
            );

            if !search_orders.is_empty() {
                let first_order = search_orders
                    .get(0)
                    .expect("First order should exist");

                ants_available.remove(&first_order.pos);

                // add first order to resulting orders
                result_orders.push(first_order.clone());
            }
        }
        result_orders
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
        let mut ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![]];

        assert_eq![
            actual_orders,
            vec![left_ant.west(), right_ant.east()]
        ];
    }

    #[test]
    fn orders_limited_by_food() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -a---*--a-
             ----------",
        );
        let strategy = &GatherFood {};

        let left_ant = pos(1, 1);
        let right_ant = pos(1, 8);
        let mut ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![left_ant]];
        assert_eq![actual_orders, vec![right_ant.west()]];
    }

    #[test]
    fn orders_limited_by_ants() {
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
        let mut ants = set![ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![]];
        assert_eq![actual_orders, vec![ant.north()]];
    }

}
