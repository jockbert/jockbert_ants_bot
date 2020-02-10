use crate::strategy::search::*;
use std::collections::HashMap;

/// Breadth first search
pub struct BFS {}

impl Search for BFS {
    /// Nearest orders to 'to' from 'from', is actually a reversed breadth first
    /// search starting with 'to' and searching for the first maching positions
    /// in 'from'.
    fn search(
        &self,
        world: &dyn WorldStep,
        from: &HashSet<Position>,
        to: Position,
        max_result_len: usize,
        cutoff_len: usize,
    ) -> Vec<SearchResult> {
        // Keep track current positions to search from
        let mut fringe: HashMap<Position, SearchResult> =
            HashMap::new();

        // Keep track of the old fringe, since we want to avoid
        // searching in the wrong direction.
        let mut old_fringe: HashMap<Position, SearchResult> =
            HashMap::new();

        // Keep track of positions to use in the next iteration.
        let mut next_fringe: HashMap<Position, SearchResult> =
            HashMap::new();
        let mut results: Vec<SearchResult> = vec![];

        // Add end position to fringe, since we start the search from there.
        fringe.insert(to.clone(), SearchResult::start(to));

        let mut search_len = 0;

        'search: while !fringe.is_empty()
            && results.len() < max_result_len
            && results.len() < from.len()
            && search_len < cutoff_len
        {
            search_len += 1;
            for (pos, sr) in &fringe {
                let next_fringe_pos: Vec<Position> = world
                    .available_directions(&pos)
                    .iter()
                    .map(|dir| pos.order(*dir))
                    .map(|order| order.target_pos(world.size()))
                    .filter(|target| {
                        !old_fringe.contains_key(&target)
                            && !fringe.contains_key(&target)
                    })
                    .collect();

                for next_pos in next_fringe_pos {
                    let next_sr = sr.add_step(next_pos.clone());

                    if from.contains(&next_pos) {
                        // Add reversed search result
                        // since we are searching backwards.
                        results.push(next_sr.reverse());

                        if results.len() >= max_result_len
                            || results.len() >= from.len()
                        {
                            // No nead to search more if we do not
                            // need any more results
                            break 'search;
                        }
                    }
                    next_fringe.insert(next_pos, next_sr);
                }
            }
            old_fringe = fringe;
            fringe = next_fringe;
            next_fringe = HashMap::new();
        }
        results
    }
}
