use crate::world_step::*;
use ants_ai_challenge_api::*;
use std::collections::HashSet;

pub struct AntCrashFilter<'a> {
    delegate: &'a mut WorldStep,
}

impl<'a> AntCrashFilter<'a> {
    pub fn new(delegate: &'a mut WorldStep) -> AntCrashFilter {
        AntCrashFilter { delegate: delegate }
    }
}

impl<'a> WorldStep for AntCrashFilter<'a> {
    fn add_order(&mut self, order: Order) -> &mut WorldStep {
        self.delegate.add_order(order);
        self
    }

    fn get_orders(&self) -> Orders {
        let mut taken_targets: HashSet<Position> = HashSet::new();

        let given_orders = self.delegate.get_orders();

        let ordered_ants: Vec<_> =
            given_orders.iter().map(|o| o.pos.clone()).collect();

        let stationary_ants: Vec<_> = self
            .all_my_ants()
            .iter()
            .filter(|ant| !ordered_ants.contains(ant))
            .cloned()
            .collect();

        for ant in stationary_ants {
            taken_targets.insert(ant);
        }
        given_orders
            .iter()
            .filter(|order| {
                let target = order.target_pos(&self.size());
                let keep_order = !taken_targets.contains(&target);
                taken_targets.insert(target);
                keep_order
            })
            .cloned()
            .collect()
    }

    fn size(&self) -> &Position {
        self.delegate.size()
    }

    fn all_my_ants(&self) -> Vec<Position> {
        self.delegate.all_my_ants()
    }

    fn available_directions(&self, p: &Position) -> Vec<Direction> {
        let known_targets: Vec<_> = self
            .delegate
            .get_orders()
            .iter()
            .map(|order| order.target_pos(&self.size()))
            .collect();

        self.delegate
            .available_directions(p)
            .iter()
            .cloned()
            .filter(|dir| {
                let dir_target =
                    p.order(*dir).target_pos(self.size());
                !known_targets.contains(&dir_target)
            })
            .collect()
    }

    fn tile(&self, pos: &Position) -> Tile {
        self.delegate.tile(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_step::basic_world_step::BasicWorldStep;

    #[test]
    fn collision_order_precedence() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "a--
             -a-",
        );
        let mut filter = AntCrashFilter::new(inner);

        let top_ant = &pos(0, 0);
        let bottom_ant = &pos(1, 1);

        // All directions are available
        assert_dirs!(filter, top_ant, North, South, East, West);

        // Add first order
        filter.add_order(top_ant.order(South));

        // West is unavailable for second ant
        assert_dirs![filter, bottom_ant, North, South, East];

        // Ignore unavailable direction
        filter.add_order(bottom_ant.order(West));

        // Only first order is available
        assert_orders!(filter, top_ant.order(South));
    }

    #[test]
    fn collision_order_precedence_2() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "a-
             -a",
        );
        let mut filter = AntCrashFilter::new(inner);

        let top_ant = &pos(0, 0);
        let bottom_ant = &pos(1, 1);

        // Add first order, targets position (1,0)
        filter.add_order(bottom_ant.order(West));

        // Neither North nor South works for top ant, since the map
        // coordinates wrap around, both leads to position (1,0).
        assert_dirs![filter, top_ant, East, West];

        // Try add invalid order
        filter.add_order(top_ant.order(South));

        // Only first order is available
        assert_orders![filter, bottom_ant.order(West)];
    }

    #[test]
    fn move_out_of_way_as_later_order() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "a-
             a-",
        );
        let mut filter = AntCrashFilter::new(inner);

        let top_ant = &pos(0, 0);
        let bottom_ant = &pos(1, 0);

        filter
            .add_order(top_ant.order(South))
            .add_order(bottom_ant.order(East));

        // No orders are filtered out since the first
        // order is possible after the second order is
        // executed.
        assert_orders!(
            filter,
            top_ant.order(South),
            bottom_ant.order(East)
        );
    }

    #[test]
    fn collision_with_stationary_ant() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "aa
             aa",
        );
        let mut filter = AntCrashFilter::new(inner);

        let top_left_ant = &pos(0, 0);

        // All directions are possible since the actions of
        // the other ants are not known at the moment.
        assert_dirs!(filter, top_left_ant, North, South, East, West);

        filter.add_order(top_left_ant.order(South));

        // The added order is filtered out since no
        // of the other ants has moved.
        assert_orders!(filter);
    }

    #[test]
    fn no_ant_interference() {
        let inner = &mut BasicWorldStep::new_from_line_map(
            "a--a
             -aa-",
        );
        let mut filter = AntCrashFilter::new(inner);

        // All orders are valid and there should be no interference.
        filter
            .add_order(pos(0, 0).order(East))
            .add_order(pos(0, 3).order(West))
            .add_order(pos(1, 1).order(West))
            .add_order(pos(1, 2).order(East));

        // No orders are filtered out
        assert_orders!(
            filter,
            pos(0, 0).order(East),
            pos(0, 3).order(West),
            pos(1, 1).order(West),
            pos(1, 2).order(East)
        );
    }
}