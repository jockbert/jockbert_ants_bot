pub mod ant_crash_filter;
pub mod avoid_water_filter;
pub mod basic_world_step;

pub use ant_crash_filter::*;
pub use avoid_water_filter::*;
pub use basic_world_step::*;

#[cfg(test)]
use crate::utilities::*;
use ants_ai_challenge_api::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub enum Tile {
    Ant(u8),
    AntOnHill(u8, u8),
    Empty,
    Water,
    Food,
    Hill(u8),
}

fn add(c: char, incement: u8) -> char {
    (c as u8 + incement) as char
}

/// Game world simulation step state.
pub trait WorldStep {
    // Add an ant movement order.
    fn add_order(&mut self, order: Order) -> &mut dyn WorldStep;

    // Get all effective orders accumulated in this step state.
    fn get_orders(&self) -> Orders;

    // World size
    fn size(&self) -> &Position;

    fn all_my_ants(&self) -> Vec<Position>;

    fn available_directions(&self, pos: &Position) -> Vec<Direction>;

    fn tile(&self, pos: &Position) -> Tile;

    fn get_positions(&self, tile: Tile) -> Vec<Position>;

    fn format(&self, indent: &str, annotate: bool) -> String {
        let orders = self.get_orders();
        let mut order_map = HashMap::<Position, Direction>::new();
        for order in orders {
            order_map.insert(order.pos.clone(), order.dir);
        }

        let mut result = String::from("");
        for y in 0..self.size().row {
            result += indent;
            for x in 0..self.size().col {
                let p = pos(y, x);
                let identifier = match self.tile(&p) {
                    Tile::Ant(p) => add('a', p),
                    Tile::AntOnHill(p, _) => add('A', p),
                    Tile::Empty => '.',
                    Tile::Hill(p) => add('0', p),
                    Tile::Food => '*',
                    Tile::Water => '%',
                };
                result += &identifier.to_string();

                if annotate {
                    let annotation = match order_map.get(&p) {
                        Some(Direction::North) => '^',
                        Some(Direction::South) => 'v',
                        Some(Direction::East) => '>',
                        Some(Direction::West) => '<',
                        Some(Direction::NoDirection) => 'P',
                        None => ' ',
                    };
                    result += &annotation.to_string();
                }
            }
            result += "\n";
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_print_step() {
        let step = BasicWorldStep::new_from_line_map(
            "-%-
            **-",
        );
        assert_eq!("||. % . \n||* * . \n", step.format("||", true));
    }

    #[test]
    fn can_print_orders() {
        let mut step = BasicWorldStep::new_from_line_map("-a-b-");

        step.add_order(pos(0, 1).south());
        assert_eq!("##. av. b . \n", step.format("##", true));
    }

    #[test]
    fn can_print_hills_without_annotations() {
        let step = BasicWorldStep::new_from_line_map(
            "%%*!???...
             abcdefghij
             ABCDEFGHIJ
             0123456789",
        );

        assert_eq!(
            "||%%*.......
||abcdefghij
||ABCDEFGHIJ
||0123456789
",
            step.format("||", false)
        );
    }
}
