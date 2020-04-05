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

    fn hill(&self, pos: &Position) -> Option<u8> {
        for i in 0..self.world.hills.len() {
            if self
                .world
                .hills
                .get(i)
                .expect("a vector of hills")
                .contains(pos)
            {
                return Some(i as u8);
            }
        }
        None
    }

    #[cfg(test)]
    pub fn new_from_line_map(map: &'static str) -> BasicWorldStep {
        let world = world(map);
        let size = size_of_world(map);
        BasicWorldStep::new(world, size)
    }
}

impl WorldStep for BasicWorldStep {
    fn add_order(&mut self, order: Order) -> &mut dyn WorldStep {
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
        let hill = self.hill(pos);

        for i in 0..self.world.live_ants.len() {
            if self
                .world
                .live_ants
                .get(i)
                .expect("a vector of ants")
                .contains(pos)
            {
                let p = i as u8;
                return hill
                    .map_or(Tile::Ant(p), |h| Tile::AntOnHill(p, h));
            }
        }
        if let Some(p) = hill {
            Tile::Hill(p)
        } else if self.world.foods.contains(pos) {
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
            Tile::Hill(p) => match self.world.hills.get(p as usize) {
                Some(hills) => hills.clone(),
                None => vec![],
            },
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_get_enemy_hills() {
        let step = BasicWorldStep::new_from_line_map(
            "02-
             -1-
             2--
            ",
        );

        // All enemy hills are listed, but not my own (zero).
        // The vec item order is in:
        // 1. Enemy index order and then
        // 2. Left-to-right-on-top-to-bottom-lines read order
        //    (as parsed in the given string)
        assert_eq!(
            vec![pos(1, 1)],
            step.get_positions(Tile::Hill(1))
        );

        assert_eq!(
            vec![pos(0, 1), pos(2, 0)],
            step.get_positions(Tile::Hill(2))
        );
    }

    #[test]
    fn no_hills_at_all() {
        let step_without_hills =
            BasicWorldStep::new_from_line_map("-----");

        let empty: Vec<Position> = vec![];
        assert_eq!(
            empty,
            step_without_hills.get_positions(Tile::Hill(0))
        );
    }
}
