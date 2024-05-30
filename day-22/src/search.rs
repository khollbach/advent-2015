use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::sim::State;

// Q: can this be simplified? / improved
// * I think b/c we're pushing dup nodes into the prio Q,
//      there's a worst-case O(edges) space cxty, instead of O(nodes)
// * possible side-quest: implement a prio Q which supports priority updates

/// Find the shortest path to a winning state. Return the length of the path.
pub fn shortest_path(start: State) -> Option<u32> {
    let mut seen = HashMap::new();
    let mut to_visit = BinaryHeap::new(); // min-heap

    seen.insert(start.clone(), 0);
    to_visit.push(ToVisit {
        state: start,
        distance: 0,
    });

    while let Some(curr) = to_visit.pop() {
        // Filter out elements we've already visited.
        if curr.distance > seen[&curr.state] {
            continue;
        }

        if curr.state.is_winning() {
            return Some(curr.distance);
        }

        for e in curr.state.next_states() {
            let distance = curr.distance + e.cost;
            if !seen.contains_key(&e.state) || distance < seen[&e.state] {
                seen.insert(e.state.clone(), distance);
                to_visit.push(ToVisit {
                    state: e.state,
                    distance,
                });
            }
        }
    }

    None
}

/// Helper type to make a min-heap sorted by distance.
struct ToVisit {
    state: State,
    distance: u32,
}

impl Ord for ToVisit {
    /// Sort by distance decreasing.
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for ToVisit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ToVisit {
    /// Equivalence is up to distance.
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for ToVisit {}
