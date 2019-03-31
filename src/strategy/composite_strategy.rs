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
        ants_available: &mut HashSet<Position>,
    ) -> Orders {
        let mut result_orders: Vec<Order> = Vec::new();

        for strategy in self.strategies.iter() {
            let orders = strategy.apply(world_step, ants_available);

            for o in orders {
                result_orders.push(o);
            }
        }
        result_orders
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
        let mut ants: HashSet<Position> =
            set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &mut ants);

        assert_eq![ants, set![]];

        assert_eq![
            actual_orders,
            vec![left_ant.west(), right_ant.east()]
        ];
    }
}
