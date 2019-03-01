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

    fn get_orders(&mut self) -> Orders {
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
                let target_not_taken =
                    !taken_targets.contains(&target);

                if target_not_taken {
                    taken_targets.insert(target);
                }
                target_not_taken
            })
            .cloned()
            .collect()
    }

    fn size(&self) -> Position {
        self.delegate.size()
    }

    fn all_my_ants(&self) -> Vec<Position> {
        self.delegate.all_my_ants()
    }

    fn available_directions(&self, pos: Position) -> Vec<Direction> {
        self.delegate.available_directions(pos)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utilities::world;

    #[test]
    fn collision_order_precedence() {
        let inner = &mut BasicWorldStep::new(
            world(
                "a-
                 -a",
            ),
            pos(2, 2),
        );
        let actual = AntCrashFilter::new(inner)
            .add_order(pos(0, 0).order(South))
            .add_order(pos(1, 1).order(West))
            .get_orders();

        let expected = vec![pos(0, 0).order(South)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn collision_order_precedence_2() {
        let inner = &mut BasicWorldStep::new(
            world(
                "a-
                 -a",
            ),
            pos(2, 2),
        );
        let actual = AntCrashFilter::new(inner)
            .add_order(pos(1, 1).order(West))
            .add_order(pos(0, 0).order(South))
            .get_orders();

        let expected = vec![pos(1, 1).order(West)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn move_out_of_way_as_later_order() {
        let inner = &mut BasicWorldStep::new(
            world(
                "a-
                 a-",
            ),
            pos(2, 2),
        );

        let actual = AntCrashFilter::new(inner)
            .add_order(pos(0, 0).order(South))
            .add_order(pos(1, 0).order(East))
            .get_orders();

        let expected =
            vec![pos(0, 0).order(South), pos(1, 0).order(East)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn collision_with_stationary_ant() {
        let inner = &mut BasicWorldStep::new(
            world(
                "a-
                 a-",
            ),
            pos(2, 2),
        );

        let actual = AntCrashFilter::new(inner)
            .add_order(pos(0, 0).order(South))
            .get_orders();

        let expected: Vec<Order> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_ant_interference() {
        let inner = &mut BasicWorldStep::new(
            world(
                "a--a
                 -aa-",
            ),
            pos(2, 4),
        );
        let actual = AntCrashFilter::new(inner)
            .add_order(pos(0, 0).order(East))
            .add_order(pos(0, 3).order(West))
            .add_order(pos(1, 1).order(West))
            .add_order(pos(1, 2).order(East))
            .get_orders();

        let expected: Vec<Order> = vec![
            pos(0, 0).order(East),
            pos(0, 3).order(West),
            pos(1, 1).order(West),
            pos(1, 2).order(East),
        ];
        assert_eq!(expected, actual);
    }
}
