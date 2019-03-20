use crate::strategy::*;
use ants_ai_challenge_api::Orders;
use ants_ai_challenge_api::Position;

pub struct CompositeStrategy {
    strategies: Vec<&'static Strategy>,
}

impl CompositeStrategy {
    pub fn new() -> CompositeStrategy {
        CompositeStrategy {
            strategies: vec![&GatherFood {}, &RandomWalk {}],
        }
    }
}

impl Strategy for CompositeStrategy {
    fn apply(
        &self,
        world_step: &WorldStep,
        ants_available: &Vec<Position>,
    ) -> (Vec<Position>, Orders) {
        let mut result_orders: Vec<Order> = Vec::new();
        let mut rest_ants: Vec<Position> = ants_available.clone();

        for strategy in self.strategies.iter() {
            let (ants, orders) =
                strategy.apply(world_step, &rest_ants);

            rest_ants.clear();
            for ant in ants {
                rest_ants.push(ant);
            }

            for o in orders {
                result_orders.push(o);
            }
        }
        (vec![], result_orders)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_step::*;


    #[test]
    fn does_not_stall() {
        let world_step = &mut BasicWorldStep::new_from_line_map(
            "----------
             -*-a--a-*-
             ----------",
        );
        let strategy = &CompositeStrategy::new();

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
}
