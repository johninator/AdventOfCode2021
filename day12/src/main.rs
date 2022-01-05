mod graph;
mod reader;

use std::collections::HashMap;

fn main() {

    let mut my_graph = graph::Graph {nodes : HashMap::new(),
                                     start : String::from("start")};
    let edges = reader::read("input.txt").unwrap();

    for edge in edges {
        my_graph.add_edge(&edge[0], &edge[1]);
    }

    let paths = my_graph.dfs(false);

    println!("paths {}", paths);
}
