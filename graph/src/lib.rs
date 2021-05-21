// mod adjacency_list;
// use adjacency_list::*;
// #![feature(total_cmp)]

pub mod directed_graph;
pub mod matrix_graph;
pub mod undirected_graph;
// pub use directed_graph::*;
// pub use undirected_graph::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
