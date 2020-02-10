use super::manhattan_filter::manhattan;
use crate::strategy::search::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

/// A-star search, keeping old explored nodes and old fringe, in order to
/// reuse partial search results in search using multiple 'from' locations
/// aiming for the same 'to' location.
pub struct RepeatedAStar {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Informed {
    sr: SearchResult,
    forward_heuristic_cost: u16,
}

impl Ord for SearchResult {
    fn cmp(&self, other: &SearchResult) -> Ordering {
        // Notice the flipped ordering on astern_costs.
        // In case of a tie we compare other fields - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .order_length()
            .cmp(&self.order_length())
            .then_with(|| (&other.steps).cmp(&self.steps))
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &SearchResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Informed {
    fn new(
        result: SearchResult,
        target: &Position,
        world_size: &Position,
    ) -> Informed {
        let heuristic_cost =
            manhattan(&(result.last_step()), target, world_size);
        Informed {
            sr: result,
            forward_heuristic_cost: heuristic_cost,
        }
    }
    fn total_cost(&self) -> u16 {
        self.sr.order_length() as u16 + self.forward_heuristic_cost
    }
    fn go_forward(
        &self,
        dir: Direction,
        target: &Position,
        world_size: &Position,
    ) -> Informed {
        let order = self.sr.last_step().order(dir);
        let new_pos = order.target_pos(world_size);
        let fwd_cost = manhattan(&new_pos, target, world_size);

        Informed {
            sr: self.sr.add_step(new_pos),
            forward_heuristic_cost: fwd_cost,
        }
    }
}

impl Ord for Informed {
    fn cmp(&self, other: &Informed) -> Ordering {
        // Notice the flipped ordering on costs.
        // In case of a tie we compare other fields - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.

        // first lowest total cost
        // secondly lowest heuristic cost (closes to target)
        other
            .total_cost()
            .cmp(&self.total_cost())
            .then_with(|| {
                other
                    .forward_heuristic_cost
                    .cmp(&self.forward_heuristic_cost)
            })
            .then_with(|| self.sr.cmp(&other.sr))
    }
}

impl PartialOrd for Informed {
    fn partial_cmp(&self, other: &Informed) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RepeatedAStar {
    fn single_search(
        &self,
        world: &dyn WorldStep,
        from: Position,
        to: Position,
        cutoff_len: usize,
        visited: &mut HashMap<Position, SearchResult>,
    ) -> Option<SearchResult> {
        let size = world.size();
        let mut queue: BinaryHeap<Informed> = BinaryHeap::new();

        queue.push(Informed::new(
            SearchResult::start(from),
            &to,
            &size,
        ));

        for (_pos, sr) in visited.iter() {
            queue.push(Informed::new(sr.clone(), &to, &size));
        }

        while !queue.is_empty() {
            let informed =
                queue.pop().expect("queue should return something");

            let sr = informed.sr.clone();
            if sr.last_step() == to {
                return Some(sr);
            }

            world
                .available_directions(&sr.last_step())
                .iter()
                .map(|&dir| informed.go_forward(dir, &to, &size))
                .for_each(|targeted| {
                    let step = targeted.sr.last_step();
                    if !visited.contains_key(&step)
                        && targeted.total_cost() <= cutoff_len as u16
                    {
                        visited.insert(step, targeted.sr.clone());
                        queue.push(targeted);
                    }
                });
        }

        None
    }
}

impl Search for RepeatedAStar {
    /// Nearest orders to 'to' from 'from', is actually a reversed breadth first
    /// search starting with 'to' and searching for the first matching positions
    /// in 'from'.
    fn search(
        &self,
        world: &dyn WorldStep,
        from: &HashSet<Position>,
        to: Position,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<SearchResult> {
        let mut visited: HashMap<Position, SearchResult> =
            HashMap::new();

        let mut max_cost = cutoff_len as u16;

        let mut results: BinaryHeap<SearchResult> = BinaryHeap::new();

        let mut sorted_froms: Vec<_> = from
            .iter()
            .map(|f| {
                Informed::new(
                    SearchResult::start(f.clone()),
                    &to,
                    &world.size(),
                )
            })
            .collect();
        sorted_froms.sort_unstable();
        sorted_froms.reverse();

        'foo: for f in sorted_froms {
            if max_cost < f.total_cost() {
                continue 'foo;
            }

            if let Some(search_result) = self.single_search(
                world,
                to.clone(),
                f.sr.last_step(),
                max_cost as usize,
                &mut visited,
            ) {
                results.push(search_result.reverse());
            }

            if results.len() >= max_result_len {
                // if we have reached our max result length, we
                // can look at the give results and try to lower
                // the max_cost. No need to be higher that the
                // already given results and we are only
                // interested in even better results.

                if let Some(max_result_cost) = results
                    .iter()
                    .map(|sr| sr.order_length() as u16)
                    .max()
                {
                    max_cost =
                        std::cmp::min(max_cost, max_result_cost);
                }
            }
        }

        // We want the shortest results first
        let mut x = results.into_sorted_vec();
        x.reverse();
        x.truncate(max_result_len);
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_step::AvoidWaterFilter;

    #[test]
    fn single_from_search_default() {
        let world = &AvoidWaterFilter::new_from_line_map(
            "-----
             -b-a-
             -----",
        );
        let search = RepeatedAStar {};

        let actual = search.single_search(
            world,
            pos(1, 3),
            pos(1, 1),
            100,
            &mut HashMap::new(),
        );

        let expected = Some(
            SearchResult::start(pos(1, 3))
                .add_step(pos(1, 2))
                .add_step(pos(1, 1)),
        );

        assert_eq!(actual, expected);
    }
}
