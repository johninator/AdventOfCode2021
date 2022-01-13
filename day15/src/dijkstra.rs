use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Node {
    position: i32,
    distance: i32,
    position_prev: i32,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl Node {
    pub fn is_reachable(&self, node_neighbor: &Node, dim: usize) -> bool {
        let position_current_x = self.position % dim as i32;
        let position_current_y = self.position / dim as i32;
        let position_neighbor_x = node_neighbor.position % dim as i32;
        let position_neighbor_y = node_neighbor.position / dim as i32;

        if position_current_x == position_neighbor_x
            && (position_current_y - position_neighbor_y).abs() == 1
        {
            return true;
        } else if position_current_y == position_neighbor_y
            && (position_current_x - position_neighbor_x).abs() == 1
        {
            return true;
        }
        return false;
    }
}

pub fn dijkstra_search(cave: &Vec<i32>, dim: usize) {
    let mut nodes_unvisited: HashSet<Node> = HashSet::new();
    let mut nodes_visited: HashSet<Node> = HashSet::new();

    for i in 0..cave.len() {
        if i == 0 {
            nodes_unvisited.insert(Node {
                position: i as i32,
                distance: 0,
                position_prev: -1,
            });
        } else {
            nodes_unvisited.insert(Node {
                position: i as i32,
                distance: -1,
                position_prev: -1,
            });
        }
    }

    let mut node_current = Node {
        position: 0,
        distance: -1,
        position_prev: -1,
    };

    while node_current.position != cave.len() as i32 - 1 {

        // println!("next cycle");
        // print_distances(&nodes_unvisited, &nodes_visited, dim);
        // println!();


        node_current = get_next_node(&nodes_unvisited);
        nodes_unvisited.remove(&node_current);

        let mut nodes_delete: Vec<Node> = Vec::new();
        let mut nodes_add: Vec<Node> = Vec::new();

        println!(
            "current position {}, distance {}",
            node_current.position, node_current.distance
        );

        for node_neighbor in nodes_unvisited.iter() {
            if node_current.is_reachable(&node_neighbor, dim) {
                let distance_new = node_current.distance + cave[node_neighbor.position as usize];
                let mut node_neighbor_new = *node_neighbor;
                if node_neighbor.distance == -1 {
                    node_neighbor_new.distance = distance_new;
                    node_neighbor_new.position_prev = node_current.position;
                    nodes_delete.push(*node_neighbor);
                    nodes_add.push(node_neighbor_new);
                } else if distance_new < node_neighbor.distance {
                    node_neighbor_new.distance = distance_new;
                    node_neighbor_new.position_prev = node_current.position;
                    nodes_delete.push(*node_neighbor);
                    nodes_add.push(node_neighbor_new);
                }
            }
        }
        for node in nodes_delete {
            nodes_unvisited.remove(&node);
        }
        for node in nodes_add {
            nodes_unvisited.insert(node);
        }
        nodes_visited.insert(node_current);
    }
    println!("finished");
}

pub fn print_distances(nodes_unvisited: &HashSet<Node>, nodes_visited: &HashSet<Node>, dim: usize) {
    for row in 0..dim {
        println!();
        for col in 0..dim {
            let position: i32 = (row * dim + col) as i32;
            for node in nodes_unvisited.iter() {
                if node.position == position {
                    print!("{}, ", node.distance);
                }
            }
            for node in nodes_visited.iter() {
                if node.position == position {
                    print!("{}, ", node.distance);
                }
            }
        }
    }
}

fn get_next_node(nodes_unvisited: &HashSet<Node>) -> Node {
    let mut distance_min: i32 = -1;
    let mut node_min: Node = Node {
        position: 0,
        distance: -1,
        position_prev: -1,
    };

    for node in nodes_unvisited.iter().cloned() {
        if node.distance != -1 {
            if distance_min == -1 {
                distance_min = node.distance;
                node_min = node;
            } else if node.distance < distance_min {
                distance_min = node.distance;
                node_min = node;
            }
        }
    }
    node_min
}

fn update_distance() // node, distances, cave
{
    // update distances of all neighbors of current node
}
