use crate::strategy::*;
use std::collections::HashSet;

/// Nearest orders to 'to' from 'from', is actually a reversed breadth first
/// search starting with 'to' and searching for the first maching positions
/// in 'from'.
pub fn nearest_orders(
    world: &WorldStep,
    to: Position,
    from: &HashSet<Position>,
    max_result_len: usize,
    search_len_cuttoff: usize,
) -> Vec<Order> {
    // Keep track current positions to search from
    let mut fringe: HashSet<Position> = HashSet::new();

    // Keep track of the old fringe, since we want to avoid
    // searching in the wrong direction.
    let mut old_fringe: HashSet<Position> = HashSet::new();

    // Keep track of positions to use in the next iteration.
    let mut next_fringe: HashSet<Position> = HashSet::new();
    let mut results = vec![];

    // Add end position to fringe, since we start the search from there.
    fringe.insert(to);

    let mut search_len = 0;

    'search: while !fringe.is_empty()
        && results.len() < max_result_len
        && results.len() < from.len()
        && search_len < search_len_cuttoff
    {
        search_len += 1;
        for pos in fringe.clone() {
            let next_fringe_orders: Orders = world
                .available_directions(&pos)
                .iter()
                .map(|dir| pos.order(*dir))
                .filter(|order| {
                    let target = order.target_pos(world.size());
                    !old_fringe.contains(&target)
                        && !fringe.contains(&target)
                })
                .collect();

            for order in next_fringe_orders {
                let target = order.target_pos(world.size());

                if from.contains(&target) {
                    // Add reversed order, since we are searching backwards.
                    results.push(order.reverse(world.size()));

                    if results.len() >= max_result_len
                        || results.len() >= from.len()
                    {
                        // No nead to search more if we do not
                        // need any more results
                        break 'search;
                    }
                }
                next_fringe.insert(target);
            }
        }
        old_fringe = fringe;
        fringe = next_fringe;
        next_fringe = HashSet::new();
    }
    results
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

        let actual = nearest_orders(
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

        let actual =
            nearest_orders(world, pos(0, 0), &set![pos(1, 1)], 10, 5);
        assert_eq!(actual, vec![pos(1, 1).west()]);
    }

    #[test]
    fn obstacle_on_side() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "b-
             %a
             %%",
        );

        let actual =
            nearest_orders(world, pos(0, 0), &set![pos(1, 1)], 10, 5);
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

        let actual = nearest_orders(
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

        let actual = nearest_orders(
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

        let actual = nearest_orders(
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

        let actual = nearest_orders(
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

        let actual = nearest_orders(
            world,
            pos(1, 1),
            &set![pos(0, 4), pos(1, 5), pos(2, 4)],
            2,
            20,
        );
        assert_eq!(actual.len(), 2);
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

        let actual = nearest_orders(
            &world_step,
            pos(0, 0),
            &set![pos(0, 2)],
            2,
            20,
        );

        assert_eq![actual, vec![pos(0, 2).west()]];
    }

    //
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

        // The nearest ant is last in the list
        ants.insert(pos(0, 99));

        let actual =
            nearest_orders(&world_step, pos(0, 0), &ants, 1, 200);

        assert_eq![actual, vec![pos(0, 99).west()]];
    }
}
