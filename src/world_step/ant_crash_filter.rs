use crate::world_step::*;
use ants_ai_challenge_api::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct AntCrashFilter {
    delegate: Box<WorldStep>,
}

impl AntCrashFilter {
    pub fn new(delegate: Box<WorldStep>) -> AntCrashFilter {
        AntCrashFilter { delegate: delegate }
    }

    #[cfg(test)]
    pub fn new_from_line_map(map: &'static str) -> AntCrashFilter {
        let inner = BasicWorldStep::new_from_line_map(map);
        AntCrashFilter {
            delegate: Box::new(inner),
        }
    }
}

impl WorldStep for AntCrashFilter {
    fn add_order(&mut self, order: Order) -> &mut WorldStep {
        self.delegate.add_order(order);
        self
    }

    fn get_orders(&self) -> Orders {
        let given_orders = self.delegate.get_orders();

        let mut unmoved_ants: HashSet<Position> = HashSet::from_iter(
            self.delegate.all_my_ants().iter().cloned(),
        );

        let mut moved_ants: HashSet<Position> = HashSet::new();

        let mut awaiting_orders: HashMap<Position, Order> =
            HashMap::new();

        let mut executed_orders: Vec<Order> = vec![];

        for order in given_orders {
            if unmoved_ants.contains(&order.pos) {
                let target = order.target_pos(self.size());
                if moved_ants.contains(&target) {
                    // No chance for this order  to be executed - dropped
                } else if unmoved_ants.contains(&target) {
                    awaiting_orders
                        .entry(target.clone())
                        .or_insert(order);
                } else {
                    executed_orders.push(order.clone());
                    moved_ants.insert(target);
                    unmoved_ants.remove(&order.pos);

                    // Keep resolving old awaiting orders, who's target
                    // is the same as this (just executed) order's source.
                    let mut source = order.clone().pos;
                    while awaiting_orders.contains_key(&source) {
                        let awaiting_order =
                            awaiting_orders[&source].clone();

                        executed_orders.push(awaiting_order.clone());
                        moved_ants.insert(source);

                        source = awaiting_order.pos.clone();
                    }
                }
            } else {
                // Error handling please!
            }
        }

        executed_orders
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

    fn get_positions(&self, tile: Tile) -> Vec<Position> {
        self.delegate.get_positions(tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_order_precedence() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "a--
             -a-",
        );

        let top_ant = &pos(0, 0);
        let bottom_ant = &pos(1, 1);

        // All directions are available
        assert_dirs!(filter, top_ant, North, South, East, West);

        // Add first order
        filter.add_order(top_ant.south());

        // West is unavailable for second ant
        assert_dirs![filter, bottom_ant, North, South, East];

        // Ignore unavailable direction
        filter.add_order(bottom_ant.west());

        // Only first order is available
        assert_orders!(filter, top_ant.south());
    }

    #[test]
    fn collision_order_precedence_2() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "a-
             -a",
        );

        let top_ant = &pos(0, 0);
        let bottom_ant = &pos(1, 1);

        // Add first order, targets position (1,0)
        filter.add_order(bottom_ant.west());

        // Neither North nor South works for top ant, since the map
        // coordinates wrap around, both leads to position (1,0).
        assert_dirs![filter, top_ant, East, West];

        // Try add invalid order
        filter.add_order(top_ant.south());

        // Only first order is available
        assert_orders![filter, bottom_ant.west()];
    }

    #[test]
    fn move_out_of_way_as_later_order() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "aaaa-
            ",
        );

        filter
            .add_order(pos(0, 0).east())
            .add_order(pos(0, 1).east())
            .add_order(pos(0, 2).east())
            .add_order(pos(0, 3).east());

        // No orders are filtered out since the first
        // order is possible after the second order is
        // executed.
        assert_orders!(
            filter,
            pos(0, 3).east(),
            pos(0, 2).east(),
            pos(0, 1).east(),
            pos(0, 0).east()
        );
    }

    #[test]
    fn collision_with_stationary_ant() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "aa
             aa",
        );

        let top_left_ant = &pos(0, 0);

        // All directions are possible since the actions of
        // the other ants are not known at the moment.
        assert_dirs!(filter, top_left_ant, North, South, East, West);

        filter.add_order(top_left_ant.south());

        // The added order is filtered out since no
        // of the other ants has moved.
        assert_orders!(filter);
    }

    #[test]
    fn no_ant_interference() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "a--a
             -aa-",
        );

        // All orders are valid and there should be no interference.
        filter
            .add_order(pos(0, 0).east())
            .add_order(pos(0, 3).west())
            .add_order(pos(1, 1).west())
            .add_order(pos(1, 2).east());

        // No orders are filtered out
        assert_orders!(
            filter,
            pos(0, 0).east(),
            pos(0, 3).west(),
            pos(1, 1).west(),
            pos(1, 2).east()
        );
    }

    #[test]
    fn tripple_ant_pileup_with_moving_ant() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "aa-a
            ",
        );

        filter
            .add_order(pos(0, 3).west())
            .add_order(pos(0, 1).east())
            .add_order(pos(0, 0).east());

        assert_orders!(filter, pos(0, 3).west());
    }

    #[test]
    fn tripple_ant_pileup_with_stationary_ant() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "aaa
            ",
        );

        filter
            .add_order(pos(0, 1).east())
            .add_order(pos(0, 0).east());

        assert_orders!(filter);
    }

    /// Ants are moved in order to the right (east)
    #[test]
    fn tripple_ant_ordered_queue() {
        let mut filter = AntCrashFilter::new_from_line_map(
            "aaa-
            ",
        );

        filter
            .add_order(pos(0, 2).east())
            .add_order(pos(0, 1).east())
            .add_order(pos(0, 0).east());

        assert_orders!(
            filter,
            pos(0, 2).east(),
            pos(0, 1).east(),
            pos(0, 0).east()
        );
    }
}
