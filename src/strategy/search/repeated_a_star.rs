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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Visit {
    pos: Position,
    astern_cost: u16,
    astern_dir: Option<Direction>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TargetedVisit {
    visit: Visit,
    target: Position,
    world_size: Position,
    forward_heuristic_cost: u16,
}

impl Visit {
    fn start(pos: &Position) -> Visit {
        Visit {
            pos: pos.clone(),
            astern_cost: 0,
            astern_dir: None,
        }
    }
    fn astern_order(&self) -> Option<Order> {
        self.astern_dir.map(|dir| self.pos.order(dir))
    }

    fn with_target(
        &self,
        target: &Position,
        world_size: &Position,
    ) -> TargetedVisit {
        let heuristic_cost =
            manhattan(&(self.pos), target, world_size);
        TargetedVisit {
            visit: self.clone(),
            target: target.clone(),
            world_size: world_size.clone(),
            forward_heuristic_cost: heuristic_cost,
        }
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Visit) -> Ordering {
        // Notice the flipped ordering on astern_costs.
        // In case of a tie we compare other fields - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .astern_cost
            .cmp(&self.astern_cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.astern_dir.cmp(&other.astern_dir))
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Visit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TargetedVisit {
    fn total_cost(&self) -> u16 {
        self.visit.astern_cost + self.forward_heuristic_cost
    }
    fn go_forward(&self, dir: Direction) -> TargetedVisit {
        let order = self.visit.pos.order(dir);
        let new_pos = order.target_pos(&self.world_size);
        let fwd_cost =
            manhattan(&new_pos, &self.target, &self.world_size);

        TargetedVisit {
            visit: Visit {
                pos: new_pos,
                astern_cost: self.visit.astern_cost + 1,
                astern_dir: Some(dir.reverse()),
            },
            target: self.target.clone(),
            world_size: self.world_size.clone(),
            forward_heuristic_cost: fwd_cost,
        }
    }
}

impl Ord for TargetedVisit {
    fn cmp(&self, other: &TargetedVisit) -> Ordering {
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
            .then_with(|| self.target.cmp(&other.target))
            .then_with(|| self.world_size.cmp(&other.world_size))
            .then_with(|| self.visit.cmp(&other.visit))
    }
}

impl PartialOrd for TargetedVisit {
    fn partial_cmp(&self, other: &TargetedVisit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RepeatedAStar {
    fn single_search(
        &self,
        world: &WorldStep,
        to: Position,
        from: Position,
        cutoff_len: usize,
        visited: &mut HashMap<Position, Visit>,
    ) -> Option<Visit> {
        let size = world.size();
        let mut queue: BinaryHeap<TargetedVisit> = BinaryHeap::new();

        // searching backwards from "to" to "from".
        // Last transition adjacent to "from", is first player order to conduct.
        queue.push(Visit::start(&to).with_target(&from, &size));

        for (_pos, visit) in visited.iter() {
            queue.push(visit.with_target(&from, &size));
        }

        while !queue.is_empty() {
            let targeted_visit =
                queue.pop().expect("queue should return something");

            let visit = targeted_visit.visit.clone();
            if visit.pos == from {
                return Some(visit);
            }

            world
                .available_directions(&visit.pos)
                .iter()
                .map(|&dir| targeted_visit.go_forward(dir))
                .for_each(|targeted| {
                    if !visited.contains_key(&targeted.visit.pos)
                        && targeted.total_cost() <= cutoff_len as u16
                    {
                        visited.insert(
                            targeted.visit.pos.clone(),
                            targeted.visit.clone(),
                        );
                        queue.push(targeted.clone());
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
        world: &WorldStep,
        to: Position,
        from: &HashSet<Position>,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<Order> {
        let mut visited: HashMap<Position, Visit> = HashMap::new();

        let mut max_cost = cutoff_len as u16;

        let mut results: BinaryHeap<Visit> = BinaryHeap::new();

        let mut sorted_froms: Vec<_> = from
            .iter()
            .map(|f| Visit::start(&f).with_target(&to, &world.size()))
            .collect();
        sorted_froms.sort_unstable();
        sorted_froms.reverse();

        'foo: for f in sorted_froms {
            if max_cost < f.total_cost() {
                continue 'foo;
            }

            if let Some(v) = self.single_search(
                world,
                to.clone(),
                f.visit.pos.clone(),
                max_cost as usize,
                &mut visited,
            ) {
                results.push(v);
            }

            if results.len() >= max_result_len {
                // if we have reached our max result length, we
                // can look at the give results and try to lower
                // the max_cost. No need to be higher that the
                // already given results and we are only
                // interested in even better results.

                if let Some(max_result_cost) = results
                    .iter()
                    .map(|visit| visit.astern_cost)
                    .max()
                {
                    max_cost =
                        std::cmp::min(max_cost, max_result_cost);
                }
            }
        }

        let mut x = results
            .into_sorted_vec()
            .iter()
            .flat_map(|visit| visit.astern_order())
            .collect::<Vec<_>>();
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
             -a-b-
             -----",
        );
        let search = RepeatedAStar {};

        let actual = search.single_search(
            world,
            pos(1, 1),
            pos(1, 3),
            100,
            &mut HashMap::new(),
        );

        let expected = Some(Visit {
            pos: pos(1, 3),
            astern_cost: 2,
            astern_dir: Some(Direction::West),
        });

        assert_eq!(actual, expected);
    }
}
