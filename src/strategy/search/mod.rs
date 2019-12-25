pub mod bfs;
pub mod manhattan_filter;
pub mod repeated_a_star;

use crate::strategy::*;
pub use bfs::*;
use manhattan_filter::*;
pub use repeated_a_star::*;
use std::collections::HashSet;

pub trait Search {
    /// Search nearest orders from 'from' to 'to'. The search can
    /// be limited/scoped by number of sought results and search
    /// length (effort) cutoff.
    fn search(
        &self,
        world: &dyn WorldStep,
        from: &HashSet<Position>,
        to: Position,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<Order>;
}

/// Create default search algorithms
pub fn create_search() -> Box<dyn Search> {
    Box::new(ManhattanFilter {
        inner: Box::new(RepeatedAStar {}),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;
    use crate::world_step::{AvoidWaterFilter, BasicWorldStep};

    #[test]
    fn basics() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "b-a--
             -----",
        );

        let actual = create_search().search(
            world,
            &set![pos(0, 2)],
            pos(0, 0),
            10,
            10,
        );

        assert_eq!(actual, vec![pos(0, 2).west()]);
    }

    #[test]
    fn obstacle_above() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "b%%
             -a%",
        );

        let actual = create_search().search(
            world,
            &set![pos(1, 1)],
            pos(0, 0),
            10,
            5,
        );
        assert_eq!(actual, vec![pos(1, 1).west()]);
    }

    #[test]
    fn obstacle_on_side() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "b-
             %a
             %%",
        );

        let actual = create_search().search(
            world,
            &set![pos(1, 1)],
            pos(0, 0),
            10,
            5,
        );
        assert_eq!(actual, vec![pos(1, 1).north()]);
    }

    #[test]
    fn nearest_alternative() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "---a---
             -------
             -------
             -------
             a--b-a-
             ---a---
             -------
             -------
             -------",
        );

        let actual = create_search().search(
            world,
            &set![pos(0, 3), pos(5, 3), pos(4, 0), pos(4, 5)],
            pos(4, 3),
            10,
            10,
        );
        assert_eq!(
            actual,
            vec![
                pos(5, 3).north(),
                pos(4, 5).west(),
                pos(4, 0).east(),
                pos(0, 3).south()
            ]
        );
    }

    #[test]
    fn passing_boundaries() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "%-----
             a%----
             %-----
             ----%-
             ---%b%",
        );

        let actual = create_search().search(
            world,
            &set![pos(1, 0)],
            pos(4, 4),
            10,
            20,
        );
        assert_eq!(actual, vec![pos(1, 0).west()]);
    }

    #[test]
    fn locked_in() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "-%%-b--
             %aa%%%%
             -%%-a--
             -------
             -------
             -------
             -------
             -------
             -------
             -------",
        );

        let actual = create_search().search(
            world,
            &set![pos(1, 1), pos(1, 2), pos(2, 4)],
            pos(0, 4),
            10,
            20,
        );
        assert_eq!(actual, vec![pos(2, 4).south()]);
    }

    #[test]
    fn no_solution() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "-%--b--
             %a%----
             -%-----",
        );

        let actual = create_search().search(
            world,
            &set![pos(1, 1)],
            pos(0, 4),
            10,
            20,
        );
        assert_eq!(actual, vec![]);
    }

    /// Using three sources 'a' with the same length to target 'b'.
    /// One of the ants should be masked out, but which source to be
    /// masked out is implementation specific.
    #[test]
    fn restricted_by_max_results() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "
            %---a-
            %b---a
            %---a-",
        );

        let actual = create_search().search(
            world,
            &set![pos(0, 4), pos(1, 5), pos(2, 4)],
            pos(1, 1),
            2,
            20,
        );
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn restricted_by_cutoff_length() {
        let world =
            &AvoidWaterFilter::new_from_line_map("b--a-------");
        let from = set![pos(0, 3)];
        let search = create_search();

        let actual_with_cutoff_2 =
            search.search(world, &from, pos(0, 0), 1, 2);

        assert_eq!(actual_with_cutoff_2, vec![]);

        let actual_with_cutoff_3 =
            search.search(world, &from, pos(0, 0), 1, 3);

        assert_eq!(actual_with_cutoff_3, vec![pos(0, 3).west()]);
    }

    /// Using a sizable world, in combination of searching for
    /// more results than there are solutions, might provoke
    /// unnecessary long computations.
    ///
    /// This test should take too long time to execute if the
    /// implementation is bad.
    #[test]
    fn possible_solutions_cutoff() {
        let world = world("b-a--");
        let size = pos(32_000, 32_000);
        let world_step = BasicWorldStep::new(world, size);
        let actual = create_search().search(
            &world_step,
            &set![pos(0, 2)],
            pos(0, 0),
            2,
            20,
        );

        assert_eq![actual, vec![pos(0, 2).west()]];
    }

    /// Using sizable world together with a lot of possible
    /// from-ants. Should not generate performance problems.
    #[test]
    fn performance_test() {
        let world = world("b-------------------a--");
        let size = pos(32_000, 32_000);
        let world_step = BasicWorldStep::new(world, size);

        let mut ants: HashSet<Position> = set![];

        for col in 200..400 {
            for row in 0..20 {
                ants.insert(pos(row, col));
            }
        }

        // The nearest ants is last in the set, with a little randomness
        ants.insert(pos(0, 103));
        ants.insert(pos(0, 104));
        ants.insert(pos(0, 105));
        ants.insert(pos(0, 106));
        ants.insert(pos(0, 107));
        ants.insert(pos(0, 100));
        ants.insert(pos(0, 101));
        ants.insert(pos(0, 102));
        ants.insert(pos(0, 108));
        ants.insert(pos(0, 109));

        let actual = create_search().search(
            &world_step,
            &ants,
            pos(0, 0),
            10,
            500,
        );

        assert_eq![
            actual,
            vec![
                pos(0, 100).west(),
                pos(0, 101).west(),
                pos(0, 102).west(),
                pos(0, 103).west(),
                pos(0, 104).west(),
                pos(0, 105).west(),
                pos(0, 106).west(),
                pos(0, 107).west(),
                pos(0, 108).west(),
                pos(0, 109).west()
            ]
        ];
    }
}
