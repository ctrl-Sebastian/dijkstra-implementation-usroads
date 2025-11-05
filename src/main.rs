use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Copy, Clone)]
struct State {
    node_id: i32,
    cost: f32,
}

// This tells the heap *how* to order two States (Min-heap)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node_id == other.node_id
    }
}

impl Eq for State {}

struct Neighbor {
    destination: i32,
    length: f32,
    description: String,
}

fn load_places(file_path: &Path) -> (HashMap<i32, String>, HashMap<String, i32>) {
    let mut place_id_to_name = HashMap::new();
    let mut place_name_to_id = HashMap::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line.expect("error getting line");
        if let Some((place_id, place_name)) = line_str.split_once(',') {
            let place_id: i32 = place_id.trim().parse().expect("Invalid place ID");
            let place_name = place_name.trim().to_string();

            place_id_to_name.insert(place_id, place_name.clone());
            place_name_to_id.insert(place_name, place_id);
        }
    }
    (place_id_to_name, place_name_to_id)
}

fn load_roads(file_path: &Path) -> HashMap<i32, Vec<Neighbor>> {
    let mut graph: HashMap<i32, Vec<Neighbor>> = HashMap::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line.expect("error getting line");
        let line_data: Vec<&str> = line_str.split(',').collect();

        if line_data.len() != 4 {
            continue;
        }

        if let (Ok(place_id_1), Ok(place_id_2), Ok(length)) = (
            line_data[0].trim().parse::<i32>(),
            line_data[1].trim().parse::<i32>(),
            line_data[2].trim().parse::<f32>(),
        ) {
            let description = line_data[3].trim().to_string();
            let neighbor_1 = Neighbor {
                destination: place_id_2,
                length,
                description: description.clone(),
            };
            let neighbor_2 = Neighbor {
                destination: place_id_1,
                length,
                description: description,
            };

            graph
                .entry(place_id_1)
                .or_insert_with(Vec::new)
                .push(neighbor_1);
            graph
                .entry(place_id_2)
                .or_insert_with(Vec::new)
                .push(neighbor_2);
        }
    }

    return graph;
}

fn dijkstra(
    graph: &HashMap<i32, Vec<Neighbor>>,
    start_id: i32,
    goal_id: i32,
) -> Option<(f32, Vec<i32>)> {
    let mut distances: HashMap<i32, f32> = HashMap::new();
    let mut previous: HashMap<i32, i32> = HashMap::new();

    let mut priority_queue = BinaryHeap::new();

	// add the starting node to distances and priority queue
    distances.insert(start_id, 0.0);
    priority_queue.push(State {
        node_id: start_id,
        cost: 0.0,
    });

	// first, get the starting node
    while let Some(State { node_id, cost }) = priority_queue.pop() {
		// if the current node is the goal node, end
        if node_id == goal_id {
            break;
        }

		//
        if cost > *distances.get(&node_id).unwrap_or(&f32::INFINITY) {
            continue;
        }

		// get all neighbors of current node
        if let Some(neighbors) = graph.get(&node_id) {
            for neighbor in neighbors {
				// for each neighbor of current node, build a next State with the neighbor id and total cost
                let next = State {
                    node_id: neighbor.destination,
                    cost: cost + neighbor.length,
                };

                let is_shorter: bool = next.cost < *distances.get(&next.node_id).unwrap_or(&f32::INFINITY);

                if is_shorter {
                    distances.insert(next.node_id, next.cost);
                    previous.insert(next.node_id, node_id);
                    priority_queue.push(next);
                }
            }
        }
    }

    if !distances.contains_key(&goal_id) {
        return None;
    }

    let mut path = Vec::new();
    let mut current_id = goal_id;
    while current_id != start_id {
        path.push(current_id);
        current_id = *previous.get(&current_id).unwrap();
    }
    path.push(start_id);
    path.reverse();

    Some((distances[&goal_id], path))
}

fn print_path(
    path: Option<(f32, Vec<i32>)>,
    place_id_to_name: &HashMap<i32, String>,
    road_map: &HashMap<i32, Vec<Neighbor>>,
    start_id: i32,
    goal_id: i32,
    user_start: &String,
    user_goal: &String,
) {
    match path {
        Some((total_distance, path_vec)) => {
            for i in 0..path_vec.len() - 1 {
                let from_id = path_vec[i];
                let to_id = path_vec[i + 1];

                let from_name = place_id_to_name
                    .get(&from_id)
                    .map_or("(null)", |s| s.as_str());
                let to_name = place_id_to_name
                    .get(&to_id)
                    .map_or("(null)", |s| s.as_str());

                let mut edge_description = "???";
                let mut edge_length = 0.0;

                if let Some(neighbors) = road_map.get(&from_id) {
                    if let Some(edge) = neighbors.iter().find(|e| e.destination == to_id) {
                        edge_description = &edge.description;
                        edge_length = edge.length;
                    }
                }

                println!(
                    "  {}: {}({}) -> {}({}), {}, {:.2} mi.",
                    i + 1,
                    from_id,
                    from_name,
                    to_id,
                    to_name,
                    edge_description,
                    edge_length
                );
            }

            println!(
                "It takes {:.2} miles from {}({}) to {}({}).",
                total_distance,
                start_id,
                user_start.trim(),
                goal_id,
                user_goal.trim()
            );
        }
        None => {
            println!(
                "No path found from {} ({}) to {} ({}). They may not be connected.",
                start_id,
                user_start.trim(),
                goal_id,
                user_goal.trim()
            );
        }
    }
}

fn main() {
    let (place_id_to_name, place_name_to_id) = load_places(Path::new("data/Place.txt"));
    let road_map: HashMap<i32, Vec<Neighbor>> = load_roads(Path::new("data/Road.txt"));

    let mut user_start = String::new();
    let mut user_goal = String::new();

    println!("Enter the Source Name:");
    io::stdin()
        .read_line(&mut user_start)
        .expect("Failed to read line");

    println!("Enter the Destination Name:");
    io::stdin()
        .read_line(&mut user_goal)
        .expect("Failed to read line");

    let start_id = match place_name_to_id.get(user_start.trim()) {
        Some(id) => *id,
        None => {
            println!("Error: Start place '{}' not found.", user_start.trim());
            return;
        }
    };
    let goal_id = match place_name_to_id.get(user_goal.trim()) {
        Some(id) => *id,
        None => {
            println!("Error: Destination place '{}' not found.", user_goal.trim());
            return;
        }
    };
    println!(
        "Searching from {} ({}) to {} ({})",
        start_id,
        user_start.trim(),
        goal_id,
        user_goal.trim()
    );

    let path: Option<(f32, Vec<i32>)> = dijkstra(&road_map, start_id, goal_id);
    print_path(path, &place_id_to_name, &road_map, start_id, goal_id, &user_start, &user_goal);
}
