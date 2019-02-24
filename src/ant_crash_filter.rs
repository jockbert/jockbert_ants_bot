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

    fn target_position(&self, order: &Order) -> Position {
        match order {
            (p, South) => pos(p.row + 1, p.col),
            (p, North) => pos(
                (p.row + self.size().row - 1) % self.size().row,
                p.col,
            ),
            (p, West) => pos(
                p.row,
                (p.col + self.size().col - 1) % self.size().col,
            ),
            (p, East) => pos(p.row, p.col + 1),
        }
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
            given_orders.iter().map(|o| o.0.clone()).collect();

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
                let target = self.target_position(order);
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utilities::world;

    fn order(row: u16, col: u16, dir: Direction) -> Order {
        (pos(row, col), dir)
    }

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
            .add_order(order(0, 0, South))
            .add_order(order(1, 1, West))
            .get_orders();

        let expected = vec![order(0, 0, South)];

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
            .add_order(order(1, 1, West))
            .add_order(order(0, 0, South))
            .get_orders();

        let expected = vec![order(1, 1, West)];

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
            .add_order(order(0, 0, South))
            .add_order(order(1, 0, East))
            .get_orders();

        let expected = vec![order(0, 0, South), order(1, 0, East)];

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
            .add_order(order(0, 0, South))
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
            .add_order(order(0, 0, East))
            .add_order(order(0, 3, West))
            .add_order(order(1, 1, West))
            .add_order(order(1, 2, East))
            .get_orders();

        let expected: Vec<Order> = vec![
            order(0, 0, East),
            order(0, 3, West),
            order(1, 1, West),
            order(1, 2, East),
        ];
        assert_eq!(expected, actual);
    }
}
