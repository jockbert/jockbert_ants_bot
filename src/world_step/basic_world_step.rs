use crate::world_step::*;
use ants_ai_challenge_api::*;

pub struct BasicWorldStep {
    given_orders: Vec<Order>,
    world: WorldState,
    size: Position,
}

impl BasicWorldStep {
    pub fn new(world: WorldState, size: Position) -> BasicWorldStep {
        BasicWorldStep {
            given_orders: vec![],
            world: world,
            size: size,
        }
    }

    #[cfg(test)]
    pub fn new_from_line_map(
        multi_line_map: &'static str,
    ) -> BasicWorldStep {
        let world = world(multi_line_map);
        let size = size_of_world(multi_line_map);
        BasicWorldStep::new(world, size)
    }
}

impl WorldStep for BasicWorldStep {
    fn add_order(&mut self, order: Order) -> &mut WorldStep {
        self.given_orders.push(order);
        self
    }

    fn get_orders(&self) -> Orders {
        self.given_orders.clone()
    }

    fn size(&self) -> &Position {
        &self.size
    }

    fn all_my_ants(&self) -> Vec<Position> {
        self.world.live_ants_for_player(0)
    }

    fn available_directions(
        &self,
        _pos: &Position,
    ) -> Vec<Direction> {
        vec![North, South, East, West]
    }

    fn tile(&self, pos: &Position) -> Tile {
        if self.world.waters.contains(pos) {
            Tile::Water
        } else {
            Tile::Empty
        }
    }
}
