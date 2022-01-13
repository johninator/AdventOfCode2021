mod reader;
mod dijkstra;

fn main() {

    let mut cave = reader::read("input.txt").unwrap();

    let dim = cave.len();
    let mut cave_flat : Vec<i32> = Vec::new();

    for row in &mut cave {
        cave_flat.append(row);
    }

    println!("cave_flat {:?}", cave_flat);
    dijkstra::dijkstra_search(&cave_flat, dim);
}


