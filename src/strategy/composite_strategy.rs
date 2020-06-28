use crate::strategy::*;
use crate::world_step::WorldStep;

use std::collections::HashSet;

pub struct CompositeStrategy {
    strategies: Vec<&'static dyn Strategy>,
}

impl CompositeStrategy {
    pub fn new_with_default() -> CompositeStrategy {
        CompositeStrategy {
            strategies: vec![
                &HillRaiser {},
                &GatherFood {},
                &SpreadOutScout {},
                &RandomWalk {},
            ],
        }
    }
}

impl Strategy for CompositeStrategy {
    fn apply(
        &self,
        world_step: &dyn WorldStep,
        ants_available: &HashSet<Position>,
    ) -> Orders {
        let mut result_orders: Vec<Order> = Vec::new();
        let mut mut_ants_available: HashSet<Position> =
            ants_available.iter().cloned().collect();

        for strategy in self.strategies.iter() {
            let orders =
                strategy.apply(world_step, &mut_ants_available);

            for order in &orders {
                mut_ants_available.remove(&order.pos);
            }
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
        let strategy = &CompositeStrategy::new_with_default();

        let left_ant = pos(1, 3);
        let right_ant = pos(1, 6);
        let ants = set![left_ant.clone(), right_ant.clone()];

        let actual_orders = strategy.apply(world_step, &ants);
        assert_eq![
            actual_orders,
            vec![left_ant.west(), right_ant.east()]
        ];
    }
}
