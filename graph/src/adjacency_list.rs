use std::cell::RefCell;
use std::rc::Rc;

type Eptr<T> = Rc<RefCell<Edge<T>>>;
type EList<T> = Vec<Eptr<T>>;
type Vptr<T> = Rc<RefCell<Vertex<T>>>;
type Vlist<T> = Vec<Rc<RefCell<Vertex<T>>>>;

pub struct Edge<T> {
    data: T,
    from: Rc<RefCell<Vertex<T>>>,
    to: Rc<RefCell<Vertex<T>>>,
}

impl<T> Edge<T> {
    pub fn new(data: T, from: Vptr<T>, to: Vptr<T>) -> Self {
        Edge { data, from, to }
    }

    pub fn new_eptr(data: T, from: Vptr<T>, to: Vptr<T>) -> Eptr<T> {
        Rc::new(RefCell::new(Edge::<T>::new(data, from, to)))
    }
}

// pub struct Vertex<T> {
//     idx: usize,
//     elist: EList<T>,
// }

// impl<T> Vertex<T> {
//     pub fn new(idx: usize) -> Self {
//         Vertex {
//             idx,
//             elist: Vec::<Rc<RefCell<Edge<T>>>>::new(),
//         }
//     }

//     pub fn new_vptr(idx: usize) -> Vptr<T> {
//         Rc::new(RefCell::new(Vertex::<T>::new(idx)))
//     }

//     pub fn push_edge(&mut self, e: Eptr<T>) {
//         self.elist.push(e);
//     }
// }

pub struct AdjGraph<T> {
    vertexs: Vlist<T>,
}

impl<T> AdjGraph<T> {
    pub fn get_vertex(&self, idx: usize) -> Vptr<T> {
        Rc::clone(&self.vertexs[idx])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, std::rc::Rc<std::cell::RefCell<Vertex<T>>>> {
        self.vertexs.iter()
    }

    // pub fn push_edge(&mut self, from_idx: usize, e: Eptr<T>) {
    //     self.vlist[from_idx].borrow_mut().push_edge(e);
    // }

    // pub fn new(v: usize) -> Self {
    //     let mut vlist = Vlist::<T>::new();
    //     for i in 1..=v {
    //         vlist.push(Vertex::<T>::new_vptr(i));
    //     }

    //     AdjGraph { vlist }
    // }

    // pub fn build(v: usize, edges: Vec<(usize, usize)>) -> AdjGraph<()> {
    //     let mut g = AdjGraph::<()>::new(v);

    //     for (from, to) in edges {
    //         let e = Edge::<()>::new_eptr((), g.get_vertex(from), g.get_vertex(to));
    //         g.push_edge(from, e);
    //     }

    //     g
    // }
}

pub struct Vertex<T> {
    idx: usize,
    visited: u8,
    vlist: Vlist<T>,
    info: T,
}

impl<T> Vertex<T> {
    pub fn new(idx: usize, info: T) -> Self {
        Vertex {
            idx,
            visited: 0,
            vlist: Vec::<Vptr<T>>::new(),
            info,
        }
    }

    pub fn new_vptr(idx: usize, info: T) -> Vptr<T> {
        Rc::new(RefCell::new(Vertex::<T>::new(idx, info)))
    }

    pub fn link(&mut self, v: Vptr<T>) {
        self.vlist.push(v);
    }
}

enum Color {
    White,
    Gray,
    Black,
}
struct VInfo<P> {
    color: Color,
    discover_time: u8,
    finish_time: u8,
    depth: u8,
    ancestor: Option<P>,
}

impl<P> VInfo<P> {
    pub fn init() -> VInfo<P> {
        VInfo {
            color: Color::White,
            discover_time: 0,
            finish_time: 0,
            depth: u8::MAX,
            ancestor: None,
        }
    }
}

type NmVertex = Vertex<()>;
type NmVptr = Rc<RefCell<NmVertex>>;

impl NmVertex {
    pub fn new_nmv_ptr(idx: usize) -> NmVptr {
        Vertex::<()>::new_vptr(idx, ())
    }
}

type UndiGraph = AdjGraph<()>;
impl UndiGraph {
    pub fn new_undigraph(v: usize) -> UndiGraph {
        let mut vertexs = Vec::<NmVptr>::new();
        UndiGraph { vertexs }
    }

    pub fn build_undigraph(v: usize, edges: Vec<(usize, usize)>) -> UndiGraph {
        let mut g = UndiGraph::new_undigraph(v);

        for (from, to) in edges {
            let mut v_from = g.get_vertex(from);
            v_from.borrow_mut().link(g.get_vertex(to));

            let mut v_to = g.get_vertex(to);
            v_to.borrow_mut().link(v_from);
        }

        g
    }

    // pub fn dfs(&mut self) {

    //     fn dfs_visit(&mut self, u: NmVptr) {

    //     }
    // }
}

impl std::fmt::Display for UndiGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        for v in self.vertexs.iter() {
            write!(f, "{}: ", v.borrow().idx)?;
            for vv in v.borrow().vlist.iter() {
                write!(f, "{} ", vv.borrow().idx)?;
            }
        }

        write!(f, " ")
    }
}
