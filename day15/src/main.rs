mod reader;

fn main() {

    let mut cave = reader::read("input.txt").unwrap();

    let dim = cave.len();
    let mut cave_flat : Vec<u32> = Vec::new();

    for row in &mut cave {
        cave_flat.append(row);
    }

    println!("cave_flat {:?}", cave_flat);
}

fn djikstra_search()
{
    // init all nodes except the start node as unvisited -> nodes_unvisited
    // init distances -> distances
    // set start node -> node_start

    // perform search until goal node has been reached
    // pick node from nodes_unvisited with smallest distance
    // update distance of this node

}

fn update_distance() // node, distances, cave
{
    // update distances of all neighbors of current node
}
