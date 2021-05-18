// #![feature(total_cmp)]
use std::cell::RefCell;
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
    visited: u32,
    color: Color,
    discover_time: u32,
    finish_time: u32,
    depth: u32,
    indegree: u32,
    distance: f32,
    ancestor: Option<Vptr>,
    vlist: Vlist,
    edges: Vec<Edge>,
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
            depth: u32::MAX,
            indegree: 0,
            distance: f32::MAX,
            ancestor: None,
            vlist: Vec::<Vptr>::new(),
            edges: Vec::new(),
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
            distance: f32::MAX,
            ancestor: None,
            vlist: Vec::<Vptr>::new(),
            edges: Vec::new(),
        }
    }

    // pub fn get_name(&self) -> usize {
    //     self.idx
    // }

    pub fn new_vptr(idx: usize) -> Vptr {
        Rc::new(RefCell::new(Vertex::new(idx)))
    }

    fn link(&mut self, v: Vptr, w: f32) {
        v.borrow_mut().indegree += 1;
        self.edges.push(Edge::new(self.idx, v.borrow().idx, w));
        self.vlist.push(v);
    }

    fn reset_info(&mut self) {
        self.visited = 0;
        self.color = Color::White;
        self.discover_time = 0;
        self.finish_time = 0;
        self.depth = u32::MAX;
        self.ancestor = None;
        self.distance = f32::MAX;
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    fn is_visited(&self) -> bool {
        self.visited >= 1
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<RefCell<Vertex>>> {
        self.vlist.iter()
    }

    pub fn iter_edge(&self) -> std::slice::Iter<'_, Edge> {
        self.edges.iter()
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

#[derive(Debug, Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    weight: f32,
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        Edge { from, to, weight }
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
        self.from == other.from && self.to == other.to
            || self.from == other.to && self.to == other.from
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:2.3}", self.from, self.to, self.weight)
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
            for Edge { from, to, weight } in v.borrow().iter_edge() {
                write!(f, "{} ({:2.3})", to, weight)?;
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

    pub fn build_graph(v: usize, edges: Vec<(usize, usize, f32)>) -> DictetedGraph {
        let mut g = DictetedGraph::new(v);

        for (from, to, weight) in edges {
            g.add_edge(from, to, weight);
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

    pub fn add_edge(&mut self, from_idx: usize, to_idx: usize, weight: f32) -> bool {
        if from_idx == to_idx {
            return false;
        }

        let from = self.get_vertex(from_idx);
        let to = self.get_vertex(to_idx);

        from.borrow_mut().link(to, weight);

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
            for Edge { from, to, weight } in v.borrow().iter_edge() {
                rg.add_edge(*from, *to, *weight);
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

    pub fn query_depth(&mut self, from_idx: usize, to_idx: usize) -> u32 {
        self.breadth_first_search(from_idx);
        self.vertex_list[to_idx].borrow().depth
    }

    pub fn get_depth(&mut self, v_idx: usize) -> u32 {
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
        let mut time = 0u32;
        dfs_helper(&start, &mut time);

        for v in self.iter() {
            dfs_helper(v, &mut time);
        }

        fn dfs_helper(v: &Vptr, time: &mut u32) {
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

    pub fn topological_sort(&mut self) -> Vec<(usize, u32)> {
        let start = self.get_min_indegree_vertex();
        self.depth_first_search(start);

        let mut order = Vec::new();
        for v in self.iter() {
            order.push((v.borrow().idx, v.borrow().finish_time));
        }

        order.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        order
    }

    pub fn topological_sort_from(&mut self, idx: usize) -> Vec<(usize, u32)> {
        let start = self.get_vertex(idx);
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

    pub fn topological_sort_by_dfs(&mut self) -> VecDeque<(usize, u32)> {
        let mut data = VecDeque::<(usize, u32)>::new();

        // let mut pre = |_v, _time| {};
        let mut pre = |v: &mut Vertex, discover_time: &mut u32| {
            *discover_time += 1;
            v.discover_time = *discover_time;
        };
        let mut post = |v: &mut Vertex, fin_time: &mut u32| {
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

// shortest path algorithm
impl DictetedGraph {
    pub fn bellman_ford(&mut self, source_idx: usize) -> bool {
        self.reset_vertexs_info();
        // Initialized source vertex's distance
        self.get_vertex(source_idx).borrow_mut().distance = 0.0;

        for _ in 0..self.V() - 1 {
            for v in self.iter() {
                for e in v.borrow().iter_edge() {
                    self.relax(e);
                }
            }
        }

        for v in self.iter() {
            for Edge { from, to, weight } in v.borrow().iter_edge() {
                let source = self.get_vertex(*from);
                let sink = self.get_vertex(*to);
                if sink.borrow().distance > source.borrow().distance + weight {
                    return false;
                }
            }
        }

        true
    }

    // Only can apply to DAG
    pub fn shortest_path_find_by_sort(&mut self, source_idx: usize) {
        let sort = self.topological_sort_by_dfs();
        // println!("{:?}", sort);

        self.reset_vertexs_info();
        self.get_vertex(source_idx).borrow_mut().distance = 0.0;

        let mut iter = sort.iter();
        if let Some((source_idx, _)) = iter.find(|(idx, _)| idx == &source_idx) {
            for e in self.get_vertex(*source_idx).borrow().iter_edge() {
                self.relax(e);
            }

            for (idx, _) in iter {
                // println!("{}", idx);
                let v = self.get_vertex(*idx);
                for e in v.borrow().iter_edge() {
                    self.relax(e);
                }
            }
        };
    }

    pub fn relax(&self, e: &Edge) {
        let Edge { from, to, weight } = e;
        let source = self.get_vertex(*from);
        let sink = self.get_vertex(*to);

        if sink.borrow().distance > source.borrow().distance + weight {
            // println!(
            //     "{}.d {}, {}.d {}, w {}",
            //     from,
            //     source.borrow().distance,
            //     to,
            //     sink.borrow().distance,
            //     weight
            // );
            sink.borrow_mut().distance = source.borrow().distance + weight;
            sink.borrow_mut().ancestor = Some(source);
        }
    }

    pub fn get_shortest_path(&self, end_idx: usize) -> Vec<usize> {
        let mut path = Vec::new();

        let mut end_vertex = self.get_vertex(end_idx);
        path.push(end_vertex.borrow().idx);

        while let Some(v) = end_vertex.clone().borrow().ancestor.clone() {
            end_vertex = v.clone();
            path.push(v.borrow().idx);
        }

        path
    }
}

pub mod Record {
    #[derive(Copy, Clone, PartialEq)]
    pub struct State {
        pub distance: f32,
        pub vertex_idx: usize,
    }

    impl Eq for State {}
    impl std::cmp::Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.distance.total_cmp(&self.distance)
            //.then_with(|| self.vertex_idx.cmp(&other.vertex_idx)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
}

// dijkstra
impl DictetedGraph {
    pub fn dijkstra_shortest_path(&mut self, source_idx: usize, goal_idx: usize) -> Option<f32> {
        let mut distance = vec![f32::MAX; self.V()];
        let mut heap = BinaryHeap::new();

        use Record::State;
        distance[source_idx] = 0.0;
        heap.push(State {
            distance: 0.0,
            vertex_idx: source_idx,
        });

        while let Some(State {
            distance: dist,
            vertex_idx: v_idx,
        }) = heap.pop()
        {
            if v_idx == goal_idx {
                return Some(dist);
            }
            for Edge {
                from: _,
                to,
                weight,
            } in self.get_vertex(v_idx).borrow().iter_edge()
            {
                if distance[*to] > dist + *weight {
                    distance[*to] = dist + *weight;
                    heap.push(State {
                        distance: distance[*to],
                        vertex_idx: *to,
                    })
                }
            }
        }

        None
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
        let weight = if nums.len() == 3 {
            nums[2].trim().parse::<f32>().unwrap_or(1.0)
        } else {
            1.0
        };
        g.add_edge(
            nums[0].trim().parse::<usize>().unwrap(),
            nums[1].trim().parse::<usize>().unwrap(),
            weight,
        );
    }

    Ok(g)
}

// fn t1(a: &mut Vertex) -> usize {
//     a.idx
// }

// fn t2(a: &mut Vertex) -> u32 {
//     a.visited
// }

// fn t3<'r>(v: &'r mut Vertex) -> u32 {
//     let a = t1(v);
//     let b = t2(v);
//     a as u32 + b
// }

// #[test]
// fn test() {
//     let mut v = Vertex::new(3);
//     let a = t1(&mut v);
//     let b = t2(&mut v);
//     println!("{} {}", a, b);
// }
