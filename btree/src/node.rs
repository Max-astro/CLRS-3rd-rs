use core::panic;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type NodePointer<K, V> = Rc<RefCell<Node<K, V>>>;

// #[macro_export]
// macro_rules! println_debug {
//     ( $e:stmt ) => {
//         //dbgprintln!($e);
//     };
// }

#[macro_export]
macro_rules! get_child {
    ( $node:expr, $idx:expr  ) => {
        Rc::clone(&$node.borrow().childs[$idx])
    };
}

#[macro_export]
macro_rules! debug_node {
    ( $a:expr ) => {{
        //dbgprintln!("node: {:?}", $a.borrow());
        for (idx, node) in $a.borrow().childs.iter().enumerate() {
            //dbgprintln!("child_{}: {:?}", idx, node.borrow());
            if node.borrow().get_n() != 0 {
                for (i, child) in node.borrow().childs.iter().enumerate() {
                    //dbgprintln!("child_{} sub_{}: {:?}", idx, i, child.borrow());
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! print_node {
    ( $a:expr ) => {
        let mut que = std::collections::VecDeque::new();
        que.push_back(($a.clone(), false));
        que.push_back(($a.clone(), true));
        let mut idx = 0;
        while let Some((node, ln)) = que.pop_front() {
            if !ln {
                //dbgprintln!("#{}#{} -> ", idx, node.borrow());
                idx += 1;

                if !node.borrow().isleaf() {
                    for tmp in node.borrow().childs.iter() {
                        que.push_back((tmp.clone(), false));
                    }
                }
            } else if !que.is_empty() {
                idx = 0;
                que.push_back((node.clone(), true));
                //dbgprintln!("\n-------------------------------------------");
            }
        }

        //dbgprintln!("\n\n");
    };
}

pub struct Node<K, V> {
    t: usize,
    isleaf: bool,
    items: Vec<Box<Item<K, V>>>,
    pub childs: Vec<NodePointer<K, V>>,
}

impl<K, V> Node<K, V> {
    pub fn new(t: usize) -> Node<K, V> {
        Node {
            t: t,
            isleaf: true,
            items: Vec::new(),
            childs: Vec::new(),
        }
    }

    pub fn allocate_node(t: usize) -> NodePointer<K, V> {
        Rc::new(RefCell::new(Node::new(t)))
    }

    pub fn get_n(&self) -> usize {
        self.items.len()
    }

    pub fn get_t(&self) -> usize {
        self.t
    }

    pub fn key_len(&self) -> usize {
        self.items.len()
    }

    pub fn child_len(&self) -> usize {
        self.childs.len()
    }

    pub fn isleaf(&self) -> bool {
        self.isleaf
    }

    pub fn get_nchild(&self, idx: usize) -> NodePointer<K, V> {
        Rc::clone(&self.childs[idx])
    }

    pub fn join_right(&mut self, right: &mut Node<K, V>, item: Box<Item<K, V>>) {
        self.items.push(item);
        self.items.append(&mut right.items);
        self.childs.append(&mut right.childs);
    }
}

impl<K, V> fmt::Display for Node<K, V>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: String;
        if self.items.is_empty() {
            items = "None".to_string();
        } else {
            items = self
                .items
                .iter()
                .map(|k| format!("{} ", k.key))
                .collect::<String>();
        }
        let l = if self.isleaf() { 'L' } else { 'N' };
        write!(
            f,
            // "[[ {}] |{}| `keys: {}, childs: {}`]",
            "[ {}]",
            items,
            // l,
            // self.key_len(),
            // self.child_len(),
        )
    }
}

impl<K, V> fmt::Debug for Node<K, V>
where
    K: fmt::Display,
{
    /* DEBUG */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: String;
        if self.items.is_empty() {
            items = "None".to_string();
        } else {
            items = self
                .items
                .iter()
                .map(|k| format!("{} ", k.key))
                .collect::<String>();
        }

        write!(
            f,
            "n: {}, t: {}, isleaf: {}\nitems.len: {}, childs.len: {}\nItems Key: {}\n",
            self.get_n(),
            self.get_t(),
            self.isleaf(),
            self.key_len(),
            self.child_len(),
            items
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Item<K, V> {
    key: K,
    value: V,
}

pub struct Btree<K, V> {
    root: NodePointer<K, V>,
}

impl<K, V> Btree<K, V>
where
    K: Ord + Clone + fmt::Display,
    V: Clone,
{
    pub fn new(t: usize) -> Self {
        Btree {
            root: Node::allocate_node(t),
        }
    }

    pub fn size(&self) -> usize {
        let mut count = self.get_root().borrow().get_n();
        let mut que = std::collections::VecDeque::new();
        que.push_back(self.get_root());
        while let Some(node) = que.pop_front() {
            for tmp in node.borrow().childs.iter() {
                count += tmp.borrow().get_n();
                que.push_back(tmp.clone());
            }
        }
        count
    }

    pub fn get_root(&self) -> NodePointer<K, V> {
        self.root.clone()
    }

    #[allow(dead_code)]
    pub fn search(node: NodePointer<K, V>, key: &K) -> Option<(NodePointer<K, V>, usize)> {
        for (idx, item) in node.borrow().items.iter().enumerate() {
            if &item.key == key {
                return Some((node.clone(), idx));
            } else if &item.key > key {
                if node.borrow().isleaf() || idx == 0 {
                    return None;
                } else {
                    return Btree::search(node.borrow().childs[idx - 1].clone(), key);
                }
            }
        }
        None
    }

    pub fn get_nchild(node: NodePointer<K, V>, idx: usize) -> NodePointer<K, V> {
        node.borrow().get_nchild(idx)
    }

    // idx is a full childs node need to split, i.e. node.child[i].len == 2t, node.items.len == 2t-1
    pub fn split_child(node: NodePointer<K, V>, idx: usize) {
        let ynode = Rc::clone(&node.borrow().childs[idx]);
        // construct znode: move ynode item and childs to znode
        let t = ynode.borrow().get_t();
        let isleaf = ynode.borrow().isleaf;
        let items = ynode.borrow_mut().items.split_off(t); //total 2t-1 elem
                                                           //total 2t elem
        let childs = if ynode.borrow_mut().childs.is_empty() {
            vec![]
        } else {
            ynode.borrow_mut().childs.split_off(t)
        };

        let znode: Node<K, V> = Node {
            t,
            isleaf,
            items,
            childs,
        };

        // ynode.item changed, n = t-1
        let mid_key = ynode.borrow_mut().items.pop();

        // xnode.items insert midkey, xnode.childs insert znode
        node.borrow_mut().items.insert(idx, mid_key.unwrap());
        node.borrow_mut()
            .childs
            .insert(idx + 1, Rc::new(RefCell::new(znode)));
    }

    fn insert_nonfull(node: NodePointer<K, V>, key: K, value: V) {
        let search = node
            .borrow()
            .items
            .binary_search_by_key(&&key, |item| &item.key);

        match search {
            Ok(_) => panic!("insert: key: {} alreadly exist!", key),
            Err(mut insert_pos) => {
                if node.borrow().isleaf() {
                    node.borrow_mut()
                        .items
                        .insert(insert_pos, Box::new(Item { key, value }));
                } else {
                    let child_isfull: bool;
                    {
                        let child = Btree::get_nchild(Rc::clone(&node), insert_pos);
                        if child.borrow().get_n() == child.borrow().get_t() * 2 - 1 {
                            child_isfull = true;
                        } else {
                            child_isfull = false;
                        }
                    }

                    if child_isfull {
                        Btree::split_child(Rc::clone(&node), insert_pos);
                        if &key > &node.borrow().items[insert_pos].key {
                            insert_pos += 1;
                        }
                    }

                    let child = Btree::get_nchild(node, insert_pos);
                    Btree::insert_nonfull(child, key, value);
                }
            }
        };
    }

    pub fn insert(&mut self, key: K, value: V) {
        let root = Rc::clone(&self.root);

        if root.borrow().get_n() >= root.borrow().get_t() * 2 - 1 {
            let new_root: NodePointer<K, V> = Node::allocate_node(root.borrow().get_t());
            new_root.borrow_mut().isleaf = false;
            new_root.borrow_mut().childs.push(root);
            self.root = new_root;

            Btree::split_child(Rc::clone(&self.root), 0);
        }
        Btree::insert_nonfull(Rc::clone(&self.root), key, value);
    }

    #[allow(dead_code)]
    pub fn get_min_item(node: NodePointer<K, V>) -> Option<K> {
        if node.borrow().get_n() == 0 {
            None
        } else if node.borrow().isleaf() {
            Some((node.borrow().items[0]).key.clone())
        } else {
            let min_child = Rc::clone(&node.borrow().childs[0]);
            Btree::get_min_item(min_child)
        }
    }

    pub fn delete(&mut self, key: &K) -> Option<Box<Item<K, V>>> {
        //dbgprintln!("\n\nIn delete `{}`: |>>", key);
        let item = Btree::delete_key(self.get_root(), key);
        let node = self.get_root();
        if node.borrow().get_n() == 0 {
            if let Some(child) = node.borrow_mut().childs.pop() {
                self.root = child;
            };
        }
        item
        // //dbgprintln!(" #|");
    }

    fn delete_key(node: NodePointer<K, V>, key: &K) -> Option<Box<Item<K, V>>> {
        // //dbgprintln!("\nIn delete_key: `key :{}` {}", key, node.borrow());
        let t = node.borrow().get_t();
        let bisearch = node
            .borrow()
            .items
            .as_slice()
            .binary_search_by(|probe| probe.key.cmp(key));
        match bisearch {
            Ok(idx) => {
                // case 1
                if node.borrow().isleaf() {
                    //dbgprintln!(" Case1({}) -> ", idx);
                    return Some(node.borrow_mut().items.remove(idx));
                } else {
                    // case 2
                    for i in idx..=0 {
                        let child_node = Rc::clone(&node.borrow().childs[i]);
                        if child_node.borrow().get_n() >= t {
                            // case 2a
                            //dbgprintln!(" Case2a({}) -> ", idx);
                            // pick left child's biggest key
                            let mut last_item = child_node.borrow().items
                                [child_node.borrow().items.len() - 1]
                                .clone();
                            Btree::delete_key(child_node, &last_item.key);
                            std::mem::swap(&mut node.borrow_mut().items[idx], &mut last_item);
                            return Some(last_item);
                        }
                    }

                    // case 2b
                    //dbgprintln!(" Case2b({}) -> ", idx);
                    let node_n = node.borrow().get_n();
                    for i in idx + 1..node_n {
                        let child_node = Rc::clone(&node.borrow().childs[i]);
                        if child_node.borrow().get_n() >= t {
                            // pick right child's smallest key
                            let mut last_item = child_node.borrow().items[0].clone();
                            Btree::delete_key(child_node, &last_item.key);
                            // node.borrow_mut().items[idx] = last_item;
                            std::mem::swap(&mut node.borrow_mut().items[idx], &mut last_item);
                            return Some(last_item);
                        }
                    }

                    // case 2c: all childs contain less than t items
                    //dbgprintln!(" Case2c({}) -> ", idx);
                    Btree::merge_child(&mut node.borrow_mut(), idx);
                    let left_child = get_child!(node, idx);
                    // recursivly delete left child node
                    return Btree::delete_key(left_child, key);
                }
            }
            // Not found key in this node, go deeper or end
            Err(idx) => {
                // //dbgprintln!("in match Err idx: {}", idx);
                if node.borrow().isleaf() {
                    //Key not found
                    None
                } else {
                    let child = get_child!(node, idx);
                    if child.borrow().get_n() < t {
                        if Btree::extend_child(&mut node.borrow_mut(), idx) {
                            // case 3a
                            //dbgprintln!(" Case3a({}) -> ", idx);
                            print_node!(node);
                            Btree::delete_key(get_child!(node, idx), key)
                        } else {
                            // can not borrow key, go to case 3b
                            let idx = Btree::merge_child(&mut node.borrow_mut(), idx);
                            // let idx = if idx == 0 { 0 } else { idx - 1 };
                            //dbgprintln!(" Case3b({}) {} -> ", idx, node.borrow().get_n());
                            print_node!(node);
                            Btree::delete_key(get_child!(node, idx), key)
                        }
                    } else {
                        // go deeper
                        Btree::delete_key(get_child!(node, idx), key)
                    }
                }
            }
        }
    }

    // return extended success or fail
    fn extend_child(node: &mut Node<K, V>, idx: usize) -> bool {
        let n = node.get_n();
        assert!(node.childs.len() == n + 1);

        // //dbgprintln!("In extend: `idx {}` {}", idx, node);
        let mut child = node.childs[idx].borrow_mut();
        if idx < n && node.childs[idx + 1].borrow().get_n() >= node.t {
            // borrow right
            let right_child = node.get_nchild(idx + 1);
            std::mem::swap(&mut node.items[idx], &mut right_child.borrow_mut().items[0]);

            // remove right[0], add it to child back
            let down_item = right_child.borrow_mut().items.remove(0);
            child.items.push(down_item);

            // child_left's greatest child need to be child's smallest child
            if !right_child.borrow().isleaf() {
                let descendent = right_child.borrow_mut().childs.remove(0);
                child.childs.push(descendent);
            }
        } else if idx > 0 && node.childs[idx - 1].borrow().get_n() >= node.t {
            // borrow left
            let left_child = node.get_nchild(idx - 1);
            let mut left_item = left_child.borrow_mut().items.pop().unwrap();

            std::mem::swap(&mut node.items[idx - 1], &mut left_item);
            child.items.insert(0, left_item);

            // child_left's greatest child need to be child's smallest child
            if let Some(descendent) = left_child.borrow_mut().childs.pop() {
                child.childs.insert(0, descendent);
            };

        // // take first key(smallest) of node, insert it to child(become the biggest) which need to be deleted key
        // let down_item = node.items.remove(idx - 1);
        // child.borrow_mut().items.insert(0, down_item);
        // // left node's last key (biggest), insert it to front(become the biggest) of the node.items
        // let up_item = left_child.borrow_mut().items.pop().unwrap();
        // node.items.insert(0, up_item);
        } else {
            return false;
        }

        true
    }

    fn merge_child(node: &mut Node<K, V>, idx: usize) -> usize {
        assert!(node.items.len() >= 1);
        let idx = if idx == node.items.len() {
            idx - 1
        } else {
            idx
        };
        let mid_item = node.items.remove(idx);

        let left = node.get_nchild(idx);
        let right = node.get_nchild(idx + 1);
        left.borrow_mut()
            .join_right(&mut right.borrow_mut(), mid_item);
        node.childs.remove(idx + 1);

        idx
    }
}
