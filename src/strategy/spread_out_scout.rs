use crate::strategy::search::*;
use crate::strategy::*;
use ants_ai_challenge_api::Orders;
use ants_ai_challenge_api::Position;

pub struct SpreadOutScout {}

fn grid_points(size: &Position, step: &Position) -> Vec<Position> {
    let mut result: Vec<Position> = vec![];

    for row in (step.row..size.row).step_by(step.row as usize) {
        for col in (step.col..size.col).step_by(step.col as usize) {
            result.push(pos(row, col));
        }
    }

    result
}

impl Strategy for SpreadOutScout {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants_available: &mut HashSet<Position>,
    ) -> Orders {
        let mut result_orders: Vec<Order> = Vec::new();

        for point in grid_points(world_step.size(), &pos(7, 7)) {
            // only interested in the nearest ant
            let search_orders = create_search().search(
                world_step,
                ants_available,
                point,
                1,
                30,
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
