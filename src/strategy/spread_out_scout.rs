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
        ants_available: &HashSet<Position>,
    ) -> Orders {
        best_orders_to_target(
            &grid_points(world_step.size(), &pos(7, 7)),
            world_step,
            ants_available,
            2,
            30,
        )
    }
}
