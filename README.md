# dijkstra-adjacency-list

A small Rust library that computes shortest paths with Dijkstra's algorithm over an adjacency-list graph.

## What is published

The crates.io package contains the reusable library in `src/lib.rs` only. The homework-specific command-line demo in `src/main.rs` and the local data files under `data/` stay in the repository but are excluded from the published crate to keep the package lightweight.

## Example

```rust
use std::collections::HashMap;
use dijkstra_adjacency_list::{dijkstra, Neighbor};

let mut graph: HashMap<i32, Vec<Neighbor<i32>>> = HashMap::new();
graph.insert(
    1,
    vec![Neighbor {
        destination: 2,
        length: 1.5,
        description: String::from("road 1"),
    }],
);
graph.insert(2, vec![]);

let result = dijkstra(&graph, 1, 2);
assert_eq!(result, Some((1.5, vec![1, 2])));
```

## Repository notes

The repository still includes a local demo binary for your own testing. If you want, you can keep iterating on that demo without affecting the published crate.