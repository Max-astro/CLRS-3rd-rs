use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Ptr = Rc<RefCell<TreeNode>>;
pub struct TreeNode {
    rank: usize,
    name: usize,
    father: Option<Ptr>,
}

impl TreeNode {
    fn make_set(x: usize) -> Ptr {
        let new = Rc::new(RefCell::new(TreeNode {
            rank: 0,
            name: x,
            father: None,
        }));
        let father = new.clone();
        new.borrow_mut().father = Some(father);
        new
    }

    pub fn link(a: &Ptr, b: &Ptr) -> bool {
        if a.borrow().father() == b.borrow().father() {
            println!("{} & {} alreadly linked", a.borrow().name, b.borrow().name);
            return false;
        }

        if a.borrow().rank > b.borrow().rank {
            b.borrow_mut().father = Some(Rc::clone(a));
        } else {
            if a.borrow().rank == b.borrow().rank {
                b.borrow_mut().rank += 1;
            }
            a.borrow_mut().father = Some(Rc::clone(b));
        }
        true
    }

    pub fn father(&self) -> usize {
        self.father.clone().unwrap().borrow().name
    }

    pub fn name(&self) -> usize {
        self.name
    }
}

pub struct DSForest {
    count: usize,
    nodes: HashMap<usize, Ptr>,
}

impl DSForest {
    pub fn new(i: usize) -> DSForest {
        let nodes = HashMap::new();
        let mut set = DSForest { count: 0, nodes };
        for idx in 0..i {
            set.add(idx);
        }
        set
    }

    pub fn forests(&self) -> usize {
        self.count
    }

    pub fn add(&mut self, x: usize) {
        self.nodes.insert(x, TreeNode::make_set(x));
        self.count += 1;
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let pa = self.find_set(a);
        let pb = self.find_set(b);

        if TreeNode::link(&pa, &pb) {
            self.count -= 1;
        }
    }

    pub fn is_linked(&self, a: usize, b: usize) -> bool {
        let pa = self.find_set(a);
        let pb = self.find_set(b);

        if pa.borrow().father() == pb.borrow().father() {
            true
        } else {
            false
        }
    }

    pub fn find_set(&self, x: usize) -> Ptr {
        let p = self.nodes.get(&x);
        if p.is_none() {
            panic!("Node {} doesn't exist", x);
        }

        let p = p.unwrap();
        let father = p.borrow().father();
        if p.borrow().name != father {
            p.borrow_mut().father = Some(self.find_set(father));
        }
        return p.borrow().father.clone().unwrap();
    }

    pub fn print(&mut self) {
        println!("Size: {}, Count: {}", self.nodes.len(), self.count);
        let mut res = vec![vec![]; self.nodes.len()];
        for i in 0..self.nodes.len() as usize {
            self.find_set(i);
        }

        for (k, v) in &self.nodes {
            println!("{} {}", k, v.borrow().rank);
            if let Some(father) = &v.borrow().father {
                res[*k as usize].push(father.borrow().name);
            } else {
                res[*k as usize].push(*k);
            }
            // res[*k as usize].push(v.borrow());
        }

        // for (i, v) in res.iter().enumerate() {
        //     print!("{}: ", i);
        //     for j in v {
        //         print!("{} ", j);
        //     }
        //     println!();
        // }
        // println!();

        for (name, v) in self.nodes.iter() {
            println!("{}: {}", name, self.find_set(*name).borrow().name);
        }
        println!();
    }
}
