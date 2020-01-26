use crate::strategy::search::SearchResult;
use crate::strategy::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;

/// Search from multiple origins to multiple target.
pub trait MultiSearch {
    /// Search nearest orders from 'from' to 'to'. The search can
    /// be limited/scoped by number of sought results and search
    /// length (effort) cutoff.
    fn search_all(
        &self,
        world: &dyn WorldStep,
        from: &HashSet<Position>,
        to: &HashSet<Position>,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<SearchResult>;
}

/// Create default search algorithms
pub fn create_multisearch() -> Box<dyn MultiSearch> {
    Box::new(GreedyDistance {})
}

/// Determines orders between sources and targets by greedily
/// choosing the shortest distance first until either all
/// targets or all surces are included in choosen orders.
struct GreedyDistance {}

impl GreedyDistance {
    /// Given a set of search results, greedily choose the
    /// shortest ones, where to or from is not already choosen.
    fn greedily_choose(all: Vec<SearchResult>) -> Vec<SearchResult> {
        let prio_queue = BinaryHeap::from(all);
        let mut choosen_tos = HashSet::<Position>::new();
        let mut choosen_froms = HashSet::<Position>::new();
        let mut results = Vec::<SearchResult>::new();

        for result in prio_queue {
            let to = result.last_step();
            let from = result.first_step();

            let is_from_or_to_allready_choosen = choosen_tos
                .contains(&to)
                || choosen_froms.contains(&from);

            if !is_from_or_to_allready_choosen {
                results.push(result);
                choosen_froms.insert(from);
                choosen_tos.insert(to);
            }
        }

        results
    }
}

impl MultiSearch for GreedyDistance {
    fn search_all(
        &self,
        world: &dyn WorldStep,
        froms: &HashSet<Position>,
        tos: &HashSet<Position>,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<SearchResult> {
        let single_target_search =
            crate::strategy::search::create_search();

        let all_results = tos
            .iter()
            .cloned()
            .flat_map(|to| {
                single_target_search.search(
                    world,
                    froms,
                    to,
                    max_result_len,
                    cutoff_len,
                )
            })
            .collect::<Vec<SearchResult>>();

        GreedyDistance::greedily_choose(all_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;
    use crate::world_step::AvoidWaterFilter;

    fn assert_ant_a_to_food_orders(
        map: &'static str,
        expected_orders: &'static str,
    ) {
        let expected_orders_vec = orders(expected_orders)
            .into_iter()
            .collect::<HashSet<Order>>();

        let world_step = &AvoidWaterFilter::new_from_line_map(map);

        let actual_orders = create_multisearch()
            .search_all(
                world_step,
                &positions_of('a', map),
                &positions_of('*', map),
                100,
                100,
            )
            .iter()
            .flat_map(|sr| sr.first_order(world_step.size()))
            .collect::<HashSet<Order>>();

        assert_eq![actual_orders, expected_orders_vec];
    }

    #[test]
    fn search_equal_numbers_of_food_and_ants() {
        assert_ant_a_to_food_orders(
            "-*-a*-a*-a---%",
            "--->-->--<----",
        )
    }

    #[test]
    fn search_more_food_than_ants() {
        assert_ant_a_to_food_orders(
            "-*-a*-a*-----%",
            "--->-->-------",
        )
    }

    #[test]
    fn search_less_food_than_ants() {
        assert_ant_a_to_food_orders(
            "---a*-a*-a---%",
            "--->-->-------",
        )
    }

    #[test]
    fn search_separated_food_and_ants() {
        assert_ant_a_to_food_orders(
            "-*-a*%a*-a-----%",
            "--->-->---------",
        )
    }

    #[test]
    fn search_finds_nearest_target_greedy_order() {
        assert_ant_a_to_food_orders(
            "--*---a--*a---%",
            "------<---<----",
        )
    }
}
