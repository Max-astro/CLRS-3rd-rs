use graph::directed_graph::*;

fn main() {
    let mut g = parse_graph_from_stdio().unwrap();
    print!("{}", g);

    let sort = g.topological_sort_by_dfs();
    print!("TopologicalSort: ");

    println!("\nDFS");
    for (idx, _t) in sort {
        // print!("（{} {}） ", idx, _t);
        print!("{} ", idx);
    }
    println!();

    for v in g.iter() {
        println!("{}", v.borrow());
    }

    /**/
    if g.has_loop() {
        println!("Cyclical");
    } else {
        println!("Acyclic");
    }

    /**/
    let scc = g.stronge_connected_components();
    println!("{:?}", scc);
    /**/

    g.reset_vertexs_info();
    println!("\nBFS");

    g.breadth_first_search(1);
    for v in g.iter() {
        println!("{}", v.borrow());
    }

    println!("Bellman Ford Shortest Path");
    g.bellman_ford(0);
    for v in g.iter() {
        if (v.borrow().get_distance() - f32::MAX).abs() > f32::EPSILON {
            print!("{} ({:2.3}); ", v.borrow().idx(), v.borrow().get_distance());
        }
    }
    let path = g.get_shortest_path(7);
    println!("\n{:?}", path);

    println!("TopoSort Shortest Path");
    g.shortest_path_find_by_sort(0);
    for v in g.iter() {
        if (v.borrow().get_distance() - f32::MAX).abs() > f32::EPSILON {
            print!("{} ({:2.3}); ", v.borrow().idx(), v.borrow().get_distance());
        }
    }
    let path = g.get_shortest_path(7);
    println!("\n{:?}", path);

    for i in 0..=12 {
        println!("{} ({:?})", i, g.dijkstra_shortest_path(0, i));
    }
}
