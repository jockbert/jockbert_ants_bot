pub mod bfs;

use crate::strategy::*;
use bfs::*;
use std::collections::HashSet;

pub trait Search {
    /// Search nearest orders from 'from' to 'to'. The search can
    /// be limited/scoped by number of sought results and search
    /// lenght (effort) cuttoff.
    fn search(
        &self,
        world: &WorldStep,
        to: Position,
        from: &HashSet<Position>,
        max_result_len: usize,
        search_len_cuttoff: usize,
    ) -> Vec<Order>;
}

/// Ceate default search algorithms
pub fn create_search() -> Box<Search> {
    Box::new(BFS {})
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
            pos(0, 0),
            &set![pos(0, 2)],
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
            pos(0, 0),
            &set![pos(1, 1)],
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
            pos(0, 0),
            &set![pos(1, 1)],
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
            pos(4, 3),
            &set![pos(0, 3), pos(5, 3), pos(4, 0), pos(4, 5)],
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
            pos(4, 4),
            &set![pos(1, 0)],
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
            pos(0, 4),
            &set![pos(1, 1), pos(1, 2), pos(2, 4)],
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
            pos(0, 4),
            &set![pos(1, 1)],
            10,
            20,
        );
        assert_eq!(actual, vec![]);
    }

    /// Using three sources 'a' with the same length to target 'b'.
    /// One of the ants should be masked out, but which sorce to be
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
            pos(1, 1),
            &set![pos(0, 4), pos(1, 5), pos(2, 4)],
            2,
            20,
        );
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn restricted_by_cuttoff_length() {
        let world = &AvoidWaterFilter::new_from_line_map("b--a----");
        let from = set![pos(0, 3)];
        let search = create_search();

        let actual_with_cuttoff_2 =
            search.search(world, pos(0, 0), &from, 1, 2);

        assert_eq!(actual_with_cuttoff_2, vec![]);

        let actual_with_cuttoff_3 =
            search.search(world, pos(0, 0), &from, 1, 3);

        assert_eq!(actual_with_cuttoff_3, vec![pos(0, 3).west()]);
    }

    /// Using a sizable world, in combination of searching for
    /// more results than there are solutions, might provoke
    /// unnessesary long computations.
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
            pos(0, 0),
            &set![pos(0, 2)],
            2,
            20,
        );

        assert_eq![actual, vec![pos(0, 2).west()]];
    }

    /// Using sizeable world together with alot of possible
    /// from-ants. Should not generate performance problems.
    #[test]
    fn performance_test() {
        let world = world("b-------------------a--");
        let size = pos(32_000, 32_000);
        let world_step = BasicWorldStep::new(world, size);

        let mut ants: HashSet<Position> = set![];

        for col in 100..200 {
            for row in 0..20 {
                ants.insert(pos(row, col));
            }
        }

        // The nearest ant is last in the set
        ants.insert(pos(0, 99));

        let actual = create_search().search(
            &world_step,
            pos(0, 0),
            &ants,
            1,
            200,
        );

        assert_eq![actual, vec![pos(0, 99).west()]];
    }
}
