use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
enum Color {
    White,
    Gray,
    Black,
}

type Vptr = Rc<RefCell<Vertex>>;
type Vlist = Vec<Rc<RefCell<Vertex>>>;
pub struct Vertex {
    idx: usize,
    visited: u8,
    color: Color,
    discover_time: u8,
    finish_time: u8,
    depth: u8,
    indegree: u8,
    ancestor: Option<Vptr>,
    vlist: Vlist,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ac = self
            .ancestor
            .as_ref()
            .map_or("Nan".to_string(), |v| format!("{}", v.borrow().idx));
        write!(
            f,
            "{}: {:?}, dt {}, ft {}, depth {}, in-degree {}, ancestor {}, edges {}",
            self.idx,
            self.color,
            self.discover_time,
            self.finish_time,
            self.depth,
            self.indegree,
            ac,
            self.vlist.len()
        )
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
            indegree: 0,
            ancestor: None,
            vlist: Vec::<Vptr>::new(),
        }
    }

    pub fn copy(&self) -> Self {
        Vertex {
            idx: self.idx,
            visited: self.visited,
            color: Color::White,
            discover_time: self.discover_time,
            finish_time: self.finish_time,
            depth: self.depth,
            indegree: 0,
            ancestor: None,
            vlist: Vec::<Vptr>::new(),
        }
    }

    // pub fn get_name(&self) -> usize {
    //     self.idx
    // }

    pub fn new_vptr(idx: usize) -> Vptr {
        Rc::new(RefCell::new(Vertex::new(idx)))
    }

    fn link(&mut self, v: Vptr) {
        v.borrow_mut().indegree += 1;
        self.vlist.push(v);
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

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<RefCell<Vertex>>> {
        self.vlist.iter()
    }

    pub fn dfs_traverse<F, P, L, T>(
        &mut self,
        pre: &mut F,
        post: &mut P,
        loop_handle: &mut L,
        data: &mut T,
    ) where
        F: FnMut(&mut Vertex, &mut T),
        P: FnMut(&mut Vertex, &mut T),
        L: FnMut(),
    {
        if self.color != Color::White {
            return;
        }

        self.color = Color::Gray;
        {
            pre(self, data);
        }

        for v in self.iter() {
            match v.try_borrow_mut() {
                Ok(mut u) => u.dfs_traverse(pre, post, loop_handle, data),
                Err(_e) => loop_handle(),
            }
        }

        self.color = Color::Black;
        {
            post(self, data);
        }
    }
}

pub struct Edge<T> {
    from: Vertex,
    to: Vertex,
    weight: T,
}

// impl<T> Edge<T> {
//     pub fn new() {
//         let mut heap = BinaryHeap::new();
//         heap.push(1.1);
//         heap.push(1.2);
//         heap.push(1.1);
//     }
// }

impl<T: Ord> Ord for Edge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Ord> PartialOrd for Edge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Eq for Edge<T> {}

impl<T: Ord> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
    fn ne(&self, other: &Self) -> bool {
        self.weight != other.weight
    }
}

pub struct DictetedGraph {
    e: usize,
    vertex_list: Vlist,
}

impl std::fmt::Display for DictetedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        for v in self.iter() {
            write!(f, "{}: ", v.borrow().idx)?;
            for vv in v.borrow().vlist.iter() {
                write!(f, "{} ", vv.borrow().idx)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

impl DictetedGraph {
    pub fn new(v: usize) -> DictetedGraph {
        let e = 0;
        let mut vertex_list = Vec::new();
        for i in 0..v {
            vertex_list.push(Vertex::new_vptr(i));
        }
        DictetedGraph { e, vertex_list }
    }

    pub fn build_graph(v: usize, edges: Vec<(usize, usize)>) -> DictetedGraph {
        let mut g = DictetedGraph::new(v);

        for (from, to) in edges {
            g.add_edge(from, to);
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

    pub fn add_edge(&mut self, from_idx: usize, to_idx: usize) -> bool {
        if from_idx == to_idx {
            return false;
        }

        let from = self.get_vertex(from_idx);
        let to = self.get_vertex(to_idx);

        from.borrow_mut().link(to);

        self.e += 1;
        true
    }

    pub fn get_reverse(&self) -> Self {
        let mut vertex_list = Vec::new();
        for v in self.iter() {
            vertex_list.push(Rc::new(RefCell::new(v.borrow().copy())));
        }

        let mut rg = DictetedGraph {
            e: 0,
            vertex_list: vertex_list,
        };

        for v in self.iter() {
            let to_idx = v.borrow().idx;
            for u in v.borrow().iter() {
                rg.add_edge(u.borrow().idx, to_idx);
            }
        }

        rg
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<RefCell<Vertex>>> {
        self.vertex_list.iter()
    }

    pub fn reset_vertexs_info(&mut self) {
        for v in self.iter() {
            v.borrow_mut().reset_info();
        }
    }

    fn default_loop_handle() {}
    fn default_dfs_func<T>(_v: &mut Vertex, _t: &mut T) {}
}

impl DictetedGraph {
    pub fn breadth_first_search(&mut self, v_idx: usize) {
        let mut que = std::collections::VecDeque::new();
        let root = self.get_vertex(v_idx);
        root.borrow_mut().color = Color::Gray;
        root.borrow_mut().depth = 0;
        que.push_back(root);

        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            let depth = v.borrow().depth;

            for u in v.borrow().iter() {
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

    pub fn get_min_indegree_vertex(&self) -> Vptr {
        let mut min = self.vertex_list[0].borrow().indegree;
        let mut idx = 0;
        for v in self.iter().take(1) {
            if v.borrow().indegree == 0 {
                return Rc::clone(v);
            }
            if v.borrow().indegree < min {
                idx = v.borrow().idx;
                min = v.borrow().indegree;
            }
        }
        self.get_vertex(idx)
    }

    pub fn depth_first_search(&mut self, start: Vptr) {
        let mut time = 0u8;
        dfs_helper(&start, &mut time);

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

            for u in v
                .borrow()
                .iter()
                .filter(|u| u.borrow().color == Color::White)
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
        let start = self.get_min_indegree_vertex();
        self.depth_first_search(start);

        let mut order = Vec::new();
        for v in self.iter() {
            order.push((v.borrow().idx, v.borrow().finish_time));
        }

        order.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        order
    }

    pub fn dfs_traverse<F, P, L, T>(
        &mut self,
        pre: &mut F,
        post: &mut P,
        loop_handle: &mut L,
        data: &mut T,
    ) where
        F: FnMut(&mut Vertex, &mut T),
        P: FnMut(&mut Vertex, &mut T),
        L: FnMut(),
    {
        self.reset_vertexs_info();
        for v in self.iter().filter(|u| u.borrow().color == Color::White) {
            v.borrow_mut().dfs_traverse(pre, post, loop_handle, data);
        }
    }

    pub fn topological_sort_by_dfs(&mut self) -> VecDeque<(usize, u8)> {
        let mut data = VecDeque::<(usize, u8)>::new();

        // let mut pre = |_v, _time| {};
        let mut pre = |v: &mut Vertex, discover_time: &mut u8| {
            *discover_time += 1;
            v.discover_time = *discover_time;
        };
        let mut post = |v: &mut Vertex, fin_time: &mut u8| {
            *fin_time += 1;
            v.finish_time = *fin_time;
            data.push_front((v.idx, *fin_time));
        };

        let mut time = 0;
        self.dfs_traverse(
            &mut pre,
            &mut post,
            &mut DictetedGraph::default_loop_handle,
            &mut time,
        );

        data
    }

    pub fn has_loop(&mut self) -> bool {
        let mut cycle = false;
        let mut loop_handle = || {
            cycle = true;
        };

        let mut void = ();
        self.dfs_traverse(
            &mut DictetedGraph::default_dfs_func::<()>,
            &mut DictetedGraph::default_dfs_func::<()>,
            &mut loop_handle,
            &mut void,
        );

        cycle
    }

    pub fn stronge_connected_components(&mut self) -> Vec<Vec<usize>> {
        let topo_sort = self.topological_sort_by_dfs();

        let rg = self.get_reverse();
        let mut res = vec![];

        for (idx, _) in topo_sort {
            let mut connected = vec![];
            let v = rg.get_vertex(idx);

            let mut void = ();
            let mut post = |v: &mut Vertex, _: &mut ()| {
                connected.push(v.idx);
            };
            v.borrow_mut().dfs_traverse(
                &mut DictetedGraph::default_dfs_func::<()>,
                &mut post,
                &mut DictetedGraph::default_loop_handle,
                &mut void,
            );
            if !connected.is_empty() {
                res.push(connected);
            }
        }

        res
    }
}

pub fn parse_graph_from_stdio() -> Result<DictetedGraph, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // println!("parse_graph_from_stdio:  {}", input);
    let v = input.trim().parse::<usize>().unwrap();

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let e = input.trim().parse::<usize>().unwrap();

    let mut g = DictetedGraph::new(v);
    for _ in 0..e {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        // println!("{:?}", input.split_ascii_whitespace());
        // let nums: Vec<&str> = input.trim().split(' ').collect();
        let nums: Vec<&str> = input.split_ascii_whitespace().collect();
        // println!("{:?}", nums);
        // println!("{} {}", nums[0], nums[1]);
        g.add_edge(
            nums[0].trim().parse::<usize>().unwrap(),
            nums[1].trim().parse::<usize>().unwrap(),
        );
    }

    Ok(g)
}

// fn t1(a: &mut Vertex) -> usize {
//     a.idx
// }

// fn t2(a: &mut Vertex) -> u8 {
//     a.visited
// }

// fn t3<'r>(v: &'r mut Vertex) -> u8 {
//     let a = t1(v);
//     let b = t2(v);
//     a as u8 + b
// }

// #[test]
// fn test() {
//     let mut v = Vertex::new(3);
//     let a = t1(&mut v);
//     let b = t2(&mut v);
//     println!("{} {}", a, b);
// }
