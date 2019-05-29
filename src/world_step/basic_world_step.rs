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
            world,
            size,
        }
    }

    #[cfg(test)]
    pub fn new_from_line_map(map: &'static str) -> BasicWorldStep {
        let world = world(map);
        let size = size_of_world(map);
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
        if self.world.foods.contains(pos) {
            Tile::Food
        } else if self.world.waters.contains(pos) {
            Tile::Water
        } else {
            Tile::Empty
        }
    }

    fn get_positions(&self, tile: Tile) -> Vec<Position> {
        match tile {
            Tile::Food => self.world.foods.clone(),
            Tile::Water => self.world.waters.clone(),
            _ => vec![],
        }
    }
}
