use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

extern crate disjoint_set;
use disjoint_set::DSForest;
#[derive(Eq, PartialEq, Debug)]
enum Color {
    White,
    Gray,
    Black,
}

type Vptr = Rc<RefCell<Vertex>>;
type Vlist = Vec<Rc<RefCell<Vertex>>>;
type Elist = Vec<(f32, Vptr)>;
pub struct Vertex {
    idx: usize,
    visited: u8,
    color: Color,
    discover_time: u8,
    finish_time: u8,
    depth: u8,
    ancestor: Option<Vptr>,
    edges: Elist,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ac = self
            .ancestor
            .as_ref()
            .map_or("Nan".to_string(), |v| format!("{}", v.borrow().idx));
        write!(
            f,
            "{}: {:?}, dt {}, ft {}, depth {}, ancestor {}, edges {}",
            self.idx,
            self.color,
            self.discover_time,
            self.finish_time,
            self.depth,
            ac,
            self.edges.len()
        )
    }
}

impl Eq for Vertex {
    // fn eq(&self, other: &Self) -> bool {
    //     self.idx == other.idx
    // }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl Vertex {
    fn new(idx: usize) -> Self {
        Vertex {
            idx,
            visited: 0,
            color: Color::White,
            discover_time: 0,
            finish_time: 0,
            depth: u8::MAX,
            ancestor: None,
            edges: Vec::<(f32, Vptr)>::new(),
        }
    }

    pub fn new_vptr(idx: usize) -> Vptr {
        Rc::new(RefCell::new(Vertex::new(idx)))
    }

    fn link(&mut self, weight: f32, v: Vptr) {
        self.edges.push((weight, v));
    }

    fn reset_info(&mut self) {
        self.visited = 0;
        self.color = Color::White;
        self.discover_time = 0;
        self.finish_time = 0;
        self.depth = u8::MAX;
        self.ancestor = None;
    }

    fn is_visited(&self) -> bool {
        self.visited >= 1
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (f32, Rc<RefCell<Vertex>>)> {
        self.edges.iter()
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    v1: usize,
    v2: usize,
    weight: f32,
}

impl Edge {
    pub fn new(v1: usize, v2: usize, weight: f32) -> Self {
        Edge { v1, v2, weight }
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.weight > other.weight {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.v1 == other.v1 && self.v2 == other.v2 || self.v1 == other.v2 && self.v2 == other.v1
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:2.3}", self.v1, self.v2, self.weight)
    }
}

pub struct UndiGraph {
    e: usize,
    vertex_list: Vlist,
    edge_collection: Vec<Edge>,
}

impl std::fmt::Display for UndiGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.iter() {
            write!(f, "{}: ", v.borrow().idx)?;
            for (w, vv) in v.borrow().edges.iter() {
                write!(f, "{} ({:.3}) ", vv.borrow().idx, w)?;
            }
            write!(f, "\n");
        }

        write!(f, "\n")
    }
}

impl UndiGraph {
    pub fn new(v: usize) -> UndiGraph {
        let e = 0;
        let mut vertex_list = Vec::new();
        for i in 0..v {
            vertex_list.push(Vertex::new_vptr(i));
        }
        UndiGraph {
            e,
            vertex_list,
            edge_collection: Vec::new(),
        }
    }

    pub fn build_graph(v: usize, edges: Vec<(usize, usize, f32)>) -> UndiGraph {
        let mut g = UndiGraph::new(v);

        for (from, to, w) in edges {
            g.add_edge(from, to, w);
        }

        g
    }

    pub fn E(&self) -> usize {
        self.e
    }
    pub fn V(&self) -> usize {
        self.vertex_list.len()
    }

    pub fn get_vertex(&self, idx: usize) -> Vptr {
        Rc::clone(&self.vertex_list[idx])
    }

    pub fn add_vertex(&mut self, v: Vptr) {
        if v.borrow().idx >= self.V() {
            self.vertex_list.push(v);
        }
    }

    pub fn iter_edges(&self) -> std::slice::Iter<'_, Edge> {
        self.edge_collection.iter()
    }

    pub fn add_edge(&mut self, v_idx: usize, u_idx: usize, weight: f32) -> bool {
        if v_idx > self.V() || u_idx > self.V() {
            return false;
        }

        let u = self.get_vertex(u_idx);
        let v = self.get_vertex(v_idx);

        u.borrow_mut().link(weight, self.get_vertex(v_idx));
        if v_idx != u_idx {
            v.borrow_mut().link(weight, u);
        }

        self.e += 1;
        self.edge_collection.push(Edge::new(u_idx, v_idx, weight));
        true
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<RefCell<Vertex>>> {
        self.vertex_list.iter()
    }

    pub fn reset_vertexs_info(&mut self) {
        for v in self.iter() {
            v.borrow_mut().reset_info();
        }
    }
}

impl UndiGraph {
    pub fn breadth_first_search(&mut self, v_idx: usize) {
        let mut que = std::collections::VecDeque::new();
        let root = self.get_vertex(v_idx);
        root.borrow_mut().color = Color::Gray;
        root.borrow_mut().depth = 0;
        que.push_back(root);

        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            let depth = v.borrow().depth;

            for (w, u) in v.borrow().iter() {
                if u.borrow().color == Color::White {
                    u.borrow_mut().ancestor = Some(v.clone());
                    u.borrow_mut().depth = depth + 1;
                    u.borrow_mut().color = Color::Gray;
                    que.push_back(u.clone());
                }
            }
            v.borrow_mut().color = Color::Black;
        }
    }

    pub fn query_depth(&mut self, from_idx: usize, to_idx: usize) -> u8 {
        self.breadth_first_search(from_idx);
        self.vertex_list[to_idx].borrow().depth
    }

    pub fn get_depth(&mut self, v_idx: usize) -> u8 {
        self.vertex_list[v_idx].borrow().depth
    }

    pub fn get_ancestor(&self, v_idx: usize) -> Option<Vptr> {
        self.vertex_list[v_idx].borrow().ancestor.clone()
    }

    pub fn depth_first_search(&mut self) {
        let mut time = 0u8;
        for v in self.iter() {
            dfs_helper(v, &mut time);
        }

        fn dfs_helper(v: &Vptr, time: &mut u8) {
            if v.borrow().color != Color::White {
                return;
            }

            *time += 1;
            v.borrow_mut().discover_time = *time;
            v.borrow_mut().color = Color::Gray;
            for (_, u) in v
                .borrow()
                .iter()
                .filter(|(_, u)| u.borrow().color == Color::White)
            {
                u.borrow_mut().ancestor = Some(Rc::clone(v));
                dfs_helper(u, time);
            }

            v.borrow_mut().color = Color::Black;
            *time += 1;
            v.borrow_mut().finish_time = *time;
        }
    }

    pub fn topological_sort(&mut self) -> Vec<(usize, u8)> {
        self.depth_first_search();

        let mut order = Vec::new();
        for v in self.iter() {
            order.push((v.borrow().idx, v.borrow().finish_time));
        }

        // order.sort_by(|a, b| a.1.partial_cmp(b.1));

        order
    }

    pub fn get_edges_heap_min(&self) -> BinaryHeap<&Edge> {
        let mut heap = BinaryHeap::new();
        for e in self.iter_edges() {
            heap.push(e);
        }
        heap
    }

    // pub fn get_edges_heap_min(&self) -> BinaryHeap<Reverse<&Edge>> {
    //     let mut heap = BinaryHeap::new();
    //     for e in self.iter_edges() {
    //         heap.push(Reverse(e));
    //     }
    //     heap
    // }
}

// Minimum spinning trees
impl UndiGraph {
    pub fn kerskal_mst(&self) -> Vec<Edge> {
        let mut heap = self.get_edges_heap_min();
        let mut set = DSForest::new(self.V());
        let mut mst = Vec::new();

        while let Some(e) = heap.pop() {
            let Edge { v1, v2, weight: _ } = e.clone();
            if !set.is_linked(v1, v2) {
                set.union(v1, v2);
                mst.push(e.clone());
            }

            if set.forests() == 1 {
                break;
            }
        }

        mst
    }

    pub fn prim_mst(&self) -> Vec<Edge> {
        let mut mst = Vec::new();
        let mut heap = BinaryHeap::new();

        let mut visited = vec![false; self.V()];
        visited[0] = true;
        let mut u_idx = 0;

        while mst.len() != self.V() - 1 {
            let u = self.get_vertex(u_idx);
            for (weight, v) in u.borrow().iter() {
                let v_idx = v.borrow().idx;
                if !visited[v_idx] {
                    heap.push(Edge::new(u_idx, v_idx, *weight));
                }
            }

            while let Some(e) = heap.pop() {
                let v2 = e.v2;
                if !visited[v2] {
                    visited[v2] = true;
                    mst.push(e);
                    u_idx = v2;
                    break;
                }
            }
        }

        mst
    }
}

pub fn parse_graph_from_stdio_by_edges() -> Result<UndiGraph, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // println!("parse_graph_from_stdio:  {}", input);
    let v = input.trim().parse::<usize>().unwrap();

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let e = input.trim().parse::<usize>().unwrap();

    let mut g = UndiGraph::new(v);
    for _ in 0..e {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        let nums: Vec<&str> = input.split_ascii_whitespace().collect();
        g.add_edge(
            nums[0].trim().parse::<usize>().unwrap(),
            nums[1].trim().parse::<usize>().unwrap(),
            nums[2].trim().parse::<f32>().unwrap(),
        );
    }

    Ok(g)
}
