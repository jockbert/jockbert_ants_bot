pub mod ant_crash_filter;
pub mod avoid_water_filter;
pub mod basic_world_step;

pub use ant_crash_filter::*;
pub use avoid_water_filter::*;
pub use basic_world_step::*;

#[cfg(test)]
use crate::utilities::*;
use ants_ai_challenge_api::*;

#[derive(PartialEq, Eq)]
pub enum Tile {
    Empty,
    Water,
    Food,
}

/// Game world simulation step state.
pub trait WorldStep {
    // Add an ant movement order.
    fn add_order(&mut self, order: Order) -> &mut WorldStep;

    // Get all effective orders accumulated in this step state.
    fn get_orders(&self) -> Orders;

    // World size
    fn size(&self) -> &Position;

    fn all_my_ants(&self) -> Vec<Position>;

    fn available_directions(&self, pos: &Position) -> Vec<Direction>;

    fn tile(&self, pos: &Position) -> Tile;

    fn get_positions(&self, tile: Tile) -> Vec<Position>;
}
