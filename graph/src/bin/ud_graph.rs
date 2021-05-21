use graph::undirected_graph::*;

fn main() {
    let g = parse_graph_from_stdio_by_edges().unwrap();
    print!("{}", g);

    // let mut heap = g.get_edges_heap_min();

    // while !heap.is_empty() {
    //     let top = heap.pop().unwrap();
    //     println!("{:?}", top);
    // }
    let mst = g.prim_mst();
    for e in mst {
        // let Edge { v1, v2, weight } = e;
        println!("{}", e);
    }
}
