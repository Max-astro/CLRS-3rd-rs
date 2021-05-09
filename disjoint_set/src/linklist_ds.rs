use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Header = Rc<RefCell<DSHeader>>;
type Node = Rc<RefCell<ItemNode>>;
pub struct ItemNode {
    header_ptr: Header,
    next: Option<Node>,
    pub item: usize,
}

impl ItemNode {
    pub fn new_ptr(header_ptr: Header, item: usize) -> Node {
        Rc::new(RefCell::new(ItemNode {
            header_ptr,
            next: None,
            item,
        }))
    }

    pub fn get_name(&self) -> usize {
        self.item
    }

    pub fn header_ptr(&self) -> Header {
        self.header_ptr.clone()
    }

    pub fn get_last(node: Node) -> Node {
        if let Some(mut node) = node.clone().borrow().next.clone() {
            while node.borrow().next.is_some() {
                node = node.clone().borrow().next.clone().unwrap();
            }

            node
        } else {
            node
        }
    }

    pub fn set_next(&mut self, item: usize) -> Node {
        let next = ItemNode::new_ptr(self.header_ptr.clone(), item);
        self.next = Some(next.clone());
        next
    }
}

impl ItemNode {
    pub fn get_raw(&mut self) -> *mut ItemNode {
        self
    }

    pub fn iter_mut(&mut self) -> ItemIterMut<'_> {
        ItemIterMut { next: Some(self) }
    }
}
pub struct ItemIterMut<'a> {
    next: Option<&'a mut ItemNode>,
}

impl<'a> Iterator for ItemIterMut<'a> {
    type Item = &'a mut ItemNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let next = unsafe {
                node.next.clone().map(|node| {
                    let p = node.as_ptr();
                    let m: &'a mut ItemNode = &mut *p;
                    m
                })
            };
            self.next = next;
            node
        })
    }
}

pub struct DSHeader {
    name: usize,
    head: Option<Node>,
    tail: Option<Node>,
    size: usize,
}

impl DSHeader {
    pub fn new(name: usize) -> DSHeader {
        DSHeader {
            name,
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn new_ptr(name: usize) -> Header {
        Rc::new(RefCell::new(DSHeader {
            name,
            head: None,
            tail: None,
            size: 0,
        }))
    }

    pub fn add(head_ptr: &Header, item: usize) -> Node {
        let node = ItemNode::new_ptr(head_ptr.clone(), item);
        head_ptr.borrow_mut().size += 1;
        // node.borrow_mut().header_ptr = Rc::new(RefCell::new(self))
        if head_ptr.borrow().head.is_some() {
            let mut head = head_ptr.borrow().head.clone().unwrap();
            while head.borrow().next.is_some() {
                head = head.clone().borrow().next.clone().unwrap();
            }
            head.borrow_mut().next = Some(node.clone());
            head_ptr.borrow_mut().tail = Some(node.clone());
        } else {
            head_ptr.borrow_mut().head = Some(node.clone());
            head_ptr.borrow_mut().tail = Some(node.clone());
        }
        node
    }

    pub fn get_head(&self) -> Node {
        self.head.clone().unwrap()
    }

    pub fn get_tail(&self) -> Node {
        self.tail.clone().unwrap()
    }

    pub fn iter_mut(&self) -> ItemIterMut<'_> {
        let ptr = self.head.clone().unwrap().as_ptr();
        let next = Some(unsafe { &mut *ptr });
        ItemIterMut { next }
    }

    /// a.size >= b.size && b.size > 0
    pub fn union(a: &Header, b: &Header) {
        for node in b.borrow().iter_mut() {
            node.header_ptr = a.clone();
        }

        let b_head = b.borrow().head.clone();
        let b_tail = b.borrow().tail.clone();
        a.borrow().get_tail().borrow_mut().next = b_head;
        a.borrow_mut().tail = b_tail;
    }
}

pub struct DisjointSet {
    pub nodes: HashMap<usize, Node>,
    pub sets: HashMap<usize, Header>,
    pub cnt: usize,
}

impl DisjointSet {
    pub fn new() -> DisjointSet {
        let sets = HashMap::new();
        let nodes = HashMap::new();
        let cnt = 0;
        DisjointSet { nodes, sets, cnt }
    }
    pub fn get_node(&self, item: usize) -> Option<&Node> {
        self.nodes.get(&item)
    }
    pub fn make_set(&mut self, item: usize) {
        self.cnt += 1;
        let new_set = DSHeader::new_ptr(item);
        let node = DSHeader::add(&new_set, item);

        self.nodes.insert(item, node);
        self.sets.insert(item, new_set);
    }
    pub fn find_set(&self, item: usize) -> Option<Header> {
        if let Some(node) = self.nodes.get(&item) {
            Some(node.borrow().header_ptr())
        } else {
            None
        }
    }
    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.nodes.get(&a);
        let b = self.nodes.get(&b);

        if a.is_none() || b.is_none() {
            println!("Union: a or b not exist");
            return;
        }

        let n1 = &a.unwrap().borrow().header_ptr();
        let n2 = &b.unwrap().borrow().header_ptr();
        if n1.borrow().size >= n2.borrow().size {
            DSHeader::union(n1, n2);
            let set_name = n2.borrow().name;
            self.sets.remove(&set_name);
        } else {
            DSHeader::union(n2, n1);
            let set_name = n1.borrow().name;
            self.sets.remove(&set_name);
        }
    }
}

impl std::fmt::Display for DisjointSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, ptr) in &self.sets {
            write!(f, "set {}： ", name)?;
            for node in ptr.borrow_mut().iter_mut() {
                write!(f, "{} ", node.item)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
