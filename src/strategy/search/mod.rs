pub mod bfs;
pub mod manhattan_filter;
//pub mod repeated_a_star;

use crate::strategy::*;
pub use bfs::*;
use manhattan_filter::*;
//pub use repeated_a_star::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SearchResult {
    steps: Vec<Position>,
}

impl SearchResult {
    pub fn start(pos: Position) -> SearchResult {
        SearchResult { steps: vec![pos] }
    }

    pub fn last_step(&self) -> Position {
        self.steps
            .last()
            .cloned()
            .expect("All search results has at least on step")
    }

    pub fn order_length(&self) -> usize {
        self.steps.len() - 1
    }

    pub fn first_order(
        &self,
        world_size: &Position,
    ) -> Option<Order> {
        let len = self.steps.len();
        if len < 2 {
            return Option::None;
        }
        let first = self.steps.get(0).expect("");
        let second = self.steps.get(1).expect("");

        let directions = vec![North, East, South, West];
        for dir in directions {
            let order = first.order(dir);
            let target = order.target_pos(world_size);

            if target == second.clone() {
                return Some(order);
            }
        }
        panic!(format!(
            concat!(
                "Inconsistent Search result. No order ",
                "found for going from {:?} to {:?}, in ",
                "world of size {:?}"
            ),
            first, second, world_size
        ));
    }

    pub fn add_step(&self, step: Position) -> SearchResult {
        let mut result = SearchResult {
            steps: self.steps.clone(),
        };

        result.steps.push(step);
        result
    }

    pub fn reverse(&self) -> SearchResult {
        let mut result = SearchResult {
            steps: self.steps.clone(),
        };
        result.steps.reverse();
        result
    }
}

/// Search from multiple origins to a singel target.
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
    ) -> Vec<SearchResult>;
}

/// Create default search algorithms
pub fn create_search() -> Box<dyn Search> {
    Box::new(ManhattanFilter {
        inner: Box::new(BFS {}),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;
    use crate::world_step::{AvoidWaterFilter, BasicWorldStep};

    fn assert_first_order_from_a_to_b(
        map: &'static str,
        expected_first_orders: &'static str,
        max_result_len: usize,
        cutoff_len: usize,
    ) {
        let froms = &positions_of('a', map);
        let tos = &positions_of('b', map);
        assert_eq!(1, tos.len(), "There can only be one 'b' in map.");
        let to = tos.iter().next().expect("The single 'b'").clone();

        let world = &AvoidWaterFilter::new_from_line_map(map);

        let actual = create_search().search(
            world,
            froms,
            to,
            max_result_len,
            cutoff_len,
        );

        let actual = actual
            .iter()
            .flat_map(|res| res.first_order(world.size()))
            .collect::<Vec<Order>>();

        let expected = orders(expected_first_orders)
            .into_iter()
            .collect::<Vec<Order>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn basics() {
        assert_first_order_from_a_to_b(
            "b-a--
             -----",
            "--<--
             -----",
            10,
            10,
        );
    }

    #[test]
    fn obstacle_above() {
        assert_first_order_from_a_to_b(
            "b%%
             -a%",
            "-%%
             -<%",
            10,
            10,
        );
    }

    #[test]
    fn obstacle_on_side() {
        assert_first_order_from_a_to_b(
            "b-
             %a
             %%",
            "--
             %^
             %%",
            10,
            10,
        );
    }

    #[test]
    fn nearest_alternative() {
        assert_first_order_from_a_to_b(
            "---a---
             -------
             -------
             -------
             a--b-a-
             ---a---
             -------
             -------
             -------",
            "---v---
             -------
             -------
             -------
             >--b-<-
             ---^---
             -------
             -------
             -------",
            10,
            10,
        );
    }

    #[test]
    fn passing_boundaries() {
        assert_first_order_from_a_to_b(
            "%-----
             a%----
             %-----
             ----%-
             ---%b%",
            "%-----
             <%----
             %-----
             ----%-
             ---%b%",
            10,
            10,
        );
    }

    #[test]
    fn locked_in() {
        assert_first_order_from_a_to_b(
            "-%%-b--
             %aa%%%%
             -%%-a--
             -------",
            "-%%-b--
             %--%%%%
             -%%-v--
             -------",
            10,
            10,
        );
    }

    #[test]
    fn no_solution() {
        assert_first_order_from_a_to_b(
            "-%--b--
             %a%----
             -%-----",
            "-%--b--
             %-%----
             -%-----",
            10,
            10,
        );
    }

    /// Using three sources 'a' with the same length to target 'b'.
    /// One of the ants should be masked out, but which source to be
    /// masked out is implementation specific.
    #[test]
    fn restricted_by_max_results() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "%---a-
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
        assert_first_order_from_a_to_b("b--a-%", "b----%", 1, 2);
        assert_first_order_from_a_to_b("b--a-%", "b--<-%", 1, 3);
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
        let world_step = BasicWorldStep::new(world, size.clone());
        let actual = create_search()
            .search(&world_step, &set![pos(0, 2)], pos(0, 0), 2, 20)
            .iter()
            .flat_map(|result| result.first_order(&size))
            .collect::<Vec<Order>>();
        assert_eq![actual, vec![pos(0, 2).west()]];
    }
}
