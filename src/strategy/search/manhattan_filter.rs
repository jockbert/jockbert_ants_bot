use crate::strategy::search::*;

pub struct ManhattanFilter {
    pub inner: Box<dyn Search>,
}

pub fn manhattan(a: &Position, b: &Position, size: &Position) -> u16 {
    let row_diff = (i64::from(a.row) - i64::from(b.row)).abs();
    let col_diff = (i64::from(a.col) - i64::from(b.col)).abs();

    let row_distance =
        std::cmp::min(row_diff, i64::from(size.row) - row_diff);
    let col_distance =
        std::cmp::min(col_diff, i64::from(size.col) - col_diff);

    (row_distance + col_distance) as u16
}

impl Search for ManhattanFilter {
    fn search(
        &self,
        world: &dyn WorldStep,
        from: &HashSet<Position>,
        to: Position,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<Order> {
        let limit = cutoff_len as u16;
        let size = world.size();
        let filtered_from: HashSet<Position> = from
            .iter()
            .filter(|&pos| manhattan(pos, &to, size) <= limit)
            .cloned()
            .collect();

        self.inner.search(
            world,
            &filtered_from,
            to,
            max_result_len,
            cutoff_len,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_step::AvoidWaterFilter;

    struct MockedInner {
        expected_to: Position,
        expected_from: HashSet<Position>,
        expected_max_result_len: usize,
        expected_cutoff_len: usize,
    }

    impl Search for MockedInner {
        fn search(
            &self,
            _world: &dyn WorldStep,
            from: &HashSet<Position>,
            to: Position,
            max_result_len: usize,
            cutoff_len: usize,
        ) -> Vec<Order> {
            assert_eq!(self.expected_to, to);
            assert_eq!(&self.expected_from, from);
            assert_eq!(self.expected_max_result_len, max_result_len);
            assert_eq!(self.expected_cutoff_len, cutoff_len);
            vec![]
        }
    }

    #[test]
    fn basics() {
        // One step from b to pos(3,0), North
        // Two steps from b to pos(1,7), South and West
        // Three steps from b to pos(0,3), East * 3
        // Four steps from b to pos(1,3), East * 3 and South
        let world = &AvoidWaterFilter::new_from_line_map(
            "b--a----
             ---a---a
             --------
             a-------",
        );

        let original_from =
            set![pos(0, 3), pos(1, 3), pos(1, 7), pos(3, 0)];
        let to = pos(0, 0);

        ManhattanFilter {
            inner: Box::new(MockedInner {
                expected_to: to.clone(),
                expected_from: set![],
                expected_max_result_len: 10,
                expected_cutoff_len: 0,
            }),
        }
        .search(world, &original_from, to.clone(), 10, 0);

        ManhattanFilter {
            inner: Box::new(MockedInner {
                expected_to: to.clone(),
                expected_from: set![pos(3, 0)],
                expected_max_result_len: 10,
                expected_cutoff_len: 1,
            }),
        }
        .search(world, &original_from, to.clone(), 10, 1);

        ManhattanFilter {
            inner: Box::new(MockedInner {
                expected_to: to.clone(),
                expected_from: set![pos(1, 7), pos(3, 0)],
                expected_max_result_len: 10,
                expected_cutoff_len: 2,
            }),
        }
        .search(world, &original_from, to.clone(), 10, 2);

        ManhattanFilter {
            inner: Box::new(MockedInner {
                expected_to: to.clone(),
                expected_from: set![pos(0, 3), pos(1, 7), pos(3, 0)],
                expected_max_result_len: 13,
                expected_cutoff_len: 3,
            }),
        }
        .search(world, &original_from, to.clone(), 13, 3);

        ManhattanFilter {
            inner: Box::new(MockedInner {
                expected_to: to.clone(),
                expected_from: original_from.clone(),
                expected_max_result_len: 11,
                expected_cutoff_len: 4,
            }),
        }
        .search(world, &original_from, to.clone(), 11, 4);
    }
}
