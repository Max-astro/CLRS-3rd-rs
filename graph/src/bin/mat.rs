use graph::matrix_graph::{helper::*, *};

fn main() {
    let g = parse_graph_to_matrix();
    // let _ = show_all_pairs_shortest_paths(&g);
    // println!("\n------------------------------------------\n");
    // let _ = faster_all_shortest_paths(&g);
    println!("\n------------------------------------------\n");
    let _ = floyd_warshall(&g);
}
