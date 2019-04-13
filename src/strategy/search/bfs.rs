use crate::strategy::search::*;

/// Breadth first search
pub struct BFS {}

impl Search for BFS {
    /// Nearest orders to 'to' from 'from', is actually a reversed breadth first
    /// search starting with 'to' and searching for the first maching positions
    /// in 'from'.
    fn search(
        &self,
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
}
