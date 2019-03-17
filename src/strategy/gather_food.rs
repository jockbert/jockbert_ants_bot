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
        ants: Vec<Position>,
    ) -> (Vec<Position>, Orders) {
        let mut result_orders: Vec<Order> = Vec::new();
        let mut rest_ants: Vec<Position> = ants;

        for food in world_step.get_positions(Tile::Food) {
            // only interrested in the nearest ant
            let search_orders = nearest_orders(
                world_step,
                food,
                rest_ants.clone(),
                1,
            );

            if !search_orders.is_empty() {
                let first_order = search_orders
                    .get(0)
                    .expect("First order should exist");

                // remove first order from rest_ants
                let index = &rest_ants
                    .iter()
                    .position(|a| a == &first_order.pos)
                    .unwrap();
                rest_ants.remove(index.clone());

                // add first order to resulting orders
                result_orders.push(first_order.clone());
            }
        }
        (rest_ants, result_orders)
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

        let (actual_ants, actual_orders) = strategy.apply(
            world_step,
            &vec![left_ant.clone(), right_ant.clone()],
        );

        assert_eq![actual_ants, vec![]];

        assert_eq![
            actual_orders,
            vec![left_ant.order(West), right_ant.order(East)]
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

        let (actual_ants, actual_orders) = strategy.apply(
            world_step,
            &vec![left_ant.clone(), right_ant.clone()],
        );

        assert_eq![actual_ants, vec![left_ant]];
        assert_eq![actual_orders, vec![right_ant.order(West)]];
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

        let (actual_ants, actual_orders) =
            strategy.apply(world_step, &vec![ant.clone()]);

        assert_eq![actual_ants, vec![]];
        assert_eq![actual_orders, vec![ant.order(North)]];
    }

}
