use crate::world_step::*;
use ants_ai_challenge_api::*;

pub struct AvoidWaterFilter<'a> {
    delegate: &'a mut WorldStep,
}

impl<'a> AvoidWaterFilter<'a> {
    pub fn new(delegate: &'a mut WorldStep) -> AvoidWaterFilter {
        AvoidWaterFilter { delegate: delegate }
    }
}

impl<'a> WorldStep for AvoidWaterFilter<'a> {
    fn add_order(&mut self, order: Order) -> &mut WorldStep {
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
    use crate::world_step::basic_world_step::BasicWorldStep;

    #[test]
    fn no_restriction() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "%-%
             *a-
             %-%",
        );
        let filter = AvoidWaterFilter::new(inner);

        // All directions are available
        assert_dirs!(filter, &pos(1, 1), North, South, East, West);
    }

    #[test]
    fn only_east() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "%%%
             %a-
             %%%",
        );
        let filter = AvoidWaterFilter::new(inner);

        // Only east is available
        assert_dirs!(filter, &pos(1, 1), East);
    }
}
