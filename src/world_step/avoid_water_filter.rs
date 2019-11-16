use crate::world_step::*;
use ants_ai_challenge_api::*;

pub struct AvoidWaterFilter {
    delegate: Box<dyn WorldStep>,
}

impl AvoidWaterFilter {
    pub fn new(delegate: Box<dyn WorldStep>) -> AvoidWaterFilter {
        AvoidWaterFilter { delegate }
    }

    #[cfg(test)]
    pub fn new_from_line_map(map: &'static str) -> AvoidWaterFilter {
        let inner = BasicWorldStep::new_from_line_map(map);
        AvoidWaterFilter::new(Box::new(inner))
    }
}

impl WorldStep for AvoidWaterFilter {
    fn add_order(&mut self, order: Order) -> &mut dyn WorldStep {
        self.delegate.add_order(order);
        self
    }

    fn get_orders(&self) -> Orders {
        self.delegate.get_orders()
    }

    fn size(&self) -> &Position {
        self.delegate.size()
    }

    fn all_my_ants(&self) -> Vec<Position> {
        self.delegate.all_my_ants()
    }

    fn available_directions(&self, p: &Position) -> Vec<Direction> {
        self.delegate
            .available_directions(p)
            .iter()
            .cloned()
            .filter(|dir| {
                let target = p.order(*dir).target_pos(self.size());
                self.tile(&target) != Tile::Water
            })
            .collect()
    }

    fn tile(&self, pos: &Position) -> Tile {
        self.delegate.tile(pos)
    }

    fn get_positions(&self, tile: Tile) -> Vec<Position> {
        self.delegate.get_positions(tile)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_restriction() {
        let filter = AvoidWaterFilter::new_from_line_map(
            "%-%
             *a-
             %-%",
        );

        // All directions are available
        assert_dirs!(filter, &pos(1, 1), North, South, East, West);
    }

    #[test]
    fn only_east() {
        let filter = AvoidWaterFilter::new_from_line_map(
            "%%%
             %a-
             %%%",
        );

        // Only east is available
        assert_dirs!(filter, &pos(1, 1), East);
    }
}
