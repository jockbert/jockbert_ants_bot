use ants_ai_challenge_api::*;

#[cfg(test)]
fn order(row: u16, col: u16, dir: Direction) -> Order {
    (pos(row, col), dir)
}

pub struct AntCrashFilter {
    given_orders: Vec<Order>,
    world: WorldState,
    size: Position,
}

use std::collections::HashSet;

impl AntCrashFilter {
    pub fn new(world: WorldState, size: Position) -> AntCrashFilter {
        AntCrashFilter {
            given_orders: vec![],
            world: world,
            size: size,
        }
    }

    #[cfg(test)]
    pub fn add_order(
        self,
        row: u16,
        col: u16,
        dir: Direction,
    ) -> Self {
        self.add(order(row, col, dir))
    }

    pub fn add(mut self, order: Order) -> Self {
        self.given_orders.push(order);
        self
    }

    fn target_position(&self, order: &Order) -> Position {
        match order {
            (p, South) => pos(p.row + 1, p.col),
            (p, North) => pos(
                (p.row + self.size.row - 1) % self.size.row,
                p.col,
            ),
            (p, West) => pos(
                p.row,
                (p.col + self.size.col - 1) % self.size.col,
            ),
            (p, East) => pos(p.row, p.col + 1),
        }
    }

    pub fn get_orders(self) -> Orders {
        let mut taken_targets: HashSet<Position> = HashSet::new();

        let ordered_ants: Vec<Position> =
            self.given_orders.iter().map(|o| o.0.clone()).collect();

        let all_my_ants = self.world.live_ants_for_player(0);

        let stationary_ants: Vec<Position> = all_my_ants
            .iter()
            .filter(|ant| !ordered_ants.contains(ant))
            .cloned()
            .collect();

        for ant in stationary_ants {
            taken_targets.insert(ant);
        }
        self.given_orders
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utilities::world;

    #[test]
    fn collision_order_precedence() {
        let actual = AntCrashFilter::new(
            world(
                "a-
                 -a",
            ),
            pos(2, 2),
        )
        .add_order(0, 0, South)
        .add_order(1, 1, West)
        .get_orders();

        let expected = vec![order(0, 0, South)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn collision_order_precedence_2() {
        let actual = AntCrashFilter::new(
            world(
                "a-
                 -a",
            ),
            pos(2, 2),
        )
        .add_order(1, 1, West)
        .add_order(0, 0, South)
        .get_orders();

        let expected = vec![order(1, 1, West)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn move_out_of_way_as_later_order() {
        let actual = AntCrashFilter::new(
            world(
                "a-
                 a-",
            ),
            pos(2, 2),
        )
        .add_order(0, 0, South)
        .add_order(1, 0, East)
        .get_orders();

        let expected = vec![order(0, 0, South), order(1, 0, East)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn collision_with_stationary_ant() {
        let actual = AntCrashFilter::new(
            world(
                "a-
             a-",
            ),
            pos(2, 2),
        )
        .add_order(0, 0, South)
        .get_orders();

        let expected: Vec<Order> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_ant_interference() {
        let actual = AntCrashFilter::new(
            world(
                "a--a
             -aa-",
            ),
            pos(2, 4),
        )
        .add_order(0, 0, East)
        .add_order(0, 3, West)
        .add_order(1, 1, West)
        .add_order(1, 2, East)
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
