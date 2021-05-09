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
    // let mut cnt = 0;
    // for cc in scc {
    //     println!("{}:", cnt);
    //     for i in cc {
    //         print!("{} ", i);
    //     }
    //     cnt += 1;
    //     println!();
    // }

    /**/
    // {
    //     println!("\nDFS 2");
    //     g.reset_vertexs_info();
    //     let sort = g.topological_sort();
    //     for (idx, _) in sort {
    //         print!("{} ", idx);
    //     }
    //     println!();

    //     for v in g.iter() {
    //         println!("{}", v.borrow());
    //     }
    // }

    /**/
    g.reset_vertexs_info();
    println!("\nBFS");

    g.breadth_first_search(1);
    for v in g.iter() {
        println!("{}", v.borrow());
    }
}
