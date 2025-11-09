use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

/// A struct representing a connection from one node to another.
#[derive(Clone)]
pub struct Neighbor<N> {
    /// The ID of the node this edge leads to.
    pub destination: N,
    /// The cost/weight of traversing this edge.
    pub length: f32,
    /// A description of the edge (e.g., "I-95 N").
    pub description: String,
}

/// Finds the shortest path in a graph using Dijkstra's algorithm.
///
/// The graph is represented as a `HashMap` where the key is the node ID
/// and the value is a `Vec` of `Neighbor` structs.
///
/// # Type Parameters
/// * `N`: The type of the node ID. Must be copyable, hashable, and equatable.
///
/// # Arguments
/// * `graph`: A reference to the adjacency map.
/// * `start_id`: The ID of the starting node.
/// * `goal_id`: The ID of the destination node.
///
/// # Returns
/// An `Option` containing `(total_cost, path_vec)` on success,
/// or `None` if no path is found.
pub fn dijkstra<N>(
    graph: &HashMap<N, Vec<Neighbor<N>>>,
    start_id: N,
    goal_id: N,
) -> Option<(f32, Vec<N>)>
where
    N: Copy + Eq + Hash, // The traits our Node ID needs
{
    // State is now a private, internal struct.
    // It's generic over the node ID type `N`.
    #[derive(Copy, Clone)]
    struct State<N> {
        node_id: N,
        cost: f32,
    }

    // --- Trait implementations for our private State ---
    impl<N: Eq> Ord for State<N> {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .partial_cmp(&self.cost)
                .unwrap_or(Ordering::Equal)
        }
    }
    impl<N: Eq> PartialOrd for State<N> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<N: Eq> PartialEq for State<N> {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost && self.node_id == other.node_id
        }
    }
    impl<N: Eq> Eq for State<N> {}
    // --- End of private State implementation ---

    let mut distances: HashMap<N, f32> = HashMap::new();
    let mut previous: HashMap<N, N> = HashMap::new();
    let mut priority_queue = BinaryHeap::new();

    distances.insert(start_id, 0.0);
    priority_queue.push(State {
        node_id: start_id,
        cost: 0.0,
    });

    while let Some(State { node_id, cost }) = priority_queue.pop() {
        if node_id == goal_id {
            break; // We found the goal!
        }

        if cost > *distances.get(&node_id).unwrap_or(&f32::INFINITY) {
            continue; // This is a stale, more expensive path
        }

        if let Some(neighbors) = graph.get(&node_id) {
            for neighbor in neighbors {
                let next = State {
                    node_id: neighbor.destination,
                    cost: cost + neighbor.length,
                };

                let is_shorter =
                    next.cost < *distances.get(&next.node_id).unwrap_or(&f32::INFINITY);

                if is_shorter {
                    distances.insert(next.node_id, next.cost);
                    previous.insert(next.node_id, node_id);
                    priority_queue.push(next);
                }
            }
        }
    }

    // Reconstruct the path
    if !distances.contains_key(&goal_id) {
        return None; // No path found
    }

    let mut path = Vec::new();
    let mut current_id = goal_id;
    while current_id != start_id {
        path.push(current_id);
        current_id = *previous.get(&current_id).unwrap(); // Safe due to "if" above
    }
    path.push(start_id);
    path.reverse();

    Some((distances[&goal_id], path))
}