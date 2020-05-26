use crate::world_step::*;
use ants_ai_challenge_api::*;

pub struct AvoidWaterFilter<T>
where
    T: WorldStep,
{
    delegate: T,
}

impl<T: WorldStep> AvoidWaterFilter<T>
where
    T: WorldStep,
{
    pub fn new(delegate: T) -> AvoidWaterFilter<T> {
        AvoidWaterFilter::<T> { delegate }
    }

}
impl AvoidWaterFilter<BasicWorldStep> {


    #[cfg(test)]
    pub fn new_from_line_map(
        map: &'static str,
    ) -> AvoidWaterFilter<BasicWorldStep> {
        let inner = BasicWorldStep::new_from_line_map(map);
        AvoidWaterFilter::new(inner)
    }
}

impl<T> WorldStep for AvoidWaterFilter<T>
where
    T: WorldStep,
{
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
        if self.tile(p) == Tile::Water {
            return Vec::new();
        }

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

    #[test]
    fn there_is_no_direction_from_water() {
        let filter = AvoidWaterFilter::new_from_line_map(
            "---
             -%-
             ---",
        );

        // No direction is available since standing on water
        assert_no_dirs!(filter, &pos(1, 1));
    }
}
