use std::collections::HashMap;
use std::collections::HashSet;

pub struct Node {
    small: bool,
    neighbors: Vec<String>,
}

pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub start: String,
}


impl Graph {
    
    pub fn print_neighbors(&self, name : &str) {
        print!("neighbors of {} :", name);
        for neighbor in self.nodes[name].neighbors.iter() {
            print!("{}, ", neighbor);
        }
        println!();
    }

    pub fn add_edge(&mut self, name_node1: &str, name_node2: &str) {
        self.add_node(name_node1);
        self.add_node(name_node2);

        self.nodes.get_mut(name_node1).unwrap().neighbors.push(name_node2.to_string());
        self.nodes.get_mut(name_node2).unwrap().neighbors.push(name_node1.to_string());

        println!("add edge between {} and {}",
                 name_node1,
                 name_node2);
    }

    fn add_node(&mut self, name: &str) {
        if !self.nodes.contains_key(name.clone()) {
            self.nodes.insert(name.to_string(),
                              Node {
                                  small: name.chars().all(char::is_lowercase),
                                  neighbors: Vec::new(),
                              },
            );
        }
    }

    pub fn dfs(&self, visit_small_caves_once: bool) -> i32 {

        // 1st entry is the next node
        // 2nd entry is the preceding node
        // 3rd entry is the nth occurence in the path
        let mut queue: Vec<(&str, &str, usize)> = vec![(&self.start, &self.start, 1)];
        let mut path : Vec<&str> = Vec::new();
        let mut num_paths = 0;

        while !queue.is_empty() {

            let node_names = queue.pop().unwrap();
            let node_name_current = node_names.0;

            if node_name_current == "end" {

                println!("final path: {:?}", path.clone());
                num_paths += 1;

                if queue.is_empty() {
                    return num_paths;
                }

                // remove as many nodes from the path to reach the current queue node
                let node_name_queue_last_prev = queue.last().unwrap().1;
                let count_queue_last_prev = queue.last().unwrap().2;

                while *path.last().unwrap() != node_name_queue_last_prev ||
                    path.iter().filter(|&&x| x == node_name_queue_last_prev).count() != count_queue_last_prev {
                    path.pop();
                }

            } else {

                path.push(node_name_current);

                let mut num_neighbors_added = 0;

                for neighbor_name in self.nodes[node_name_current].neighbors.iter() {
                    let neighbor: &Node = &self.nodes[neighbor_name];
                    let count = path.iter().filter(|&&x| x == node_name_current).count();

                    if !neighbor.small {
                        queue.push((neighbor_name, node_name_current, count));
                        num_neighbors_added += 1;
                    } else {
                        if visit_small_caves_once {
                            if path.iter().filter(|&x| x == neighbor_name).count() < 1 {
                                queue.push((neighbor_name, node_name_current, count));
                                num_neighbors_added += 1;
                            }
                        } else { // visit one small cave twice
                            let path_small_caves : Vec<&str> = path.iter().cloned()
                                .filter(|&x| char::is_lowercase(x.chars().nth(0).unwrap()))
                                .collect();
                            let set_paths : HashSet<&str> = HashSet::from_iter(path_small_caves.iter().cloned());
                            let caves_twice_count = set_paths.iter()
                                .filter(|&&x| path.iter().filter(|&&y| y == x).count() == 2)
                                .count();

                            if neighbor_name != "start" &&
                                (path.iter().filter(|&x| x == neighbor_name).count() == 0 ||
                                 path.iter().filter(|&x| x == neighbor_name).count() == 1 && caves_twice_count == 0) {

                                queue.push((neighbor_name, node_name_current, count));
                                num_neighbors_added += 1;
                            }
                        }
                    }
                }

                if num_neighbors_added == 0 && !queue.is_empty() {
                    // remove as many nodes from the path to reach the current queue node
                    let node_name_queue_last_prev = queue.last().unwrap().1;
                    let count_queue_last_prev = queue.last().unwrap().2;

                    while *path.last().unwrap() != node_name_queue_last_prev ||
                        path.iter().filter(|&&x| x == node_name_queue_last_prev).count() != count_queue_last_prev {
                        path.pop();
                    }
                }
            }
        }
        num_paths
    }
}