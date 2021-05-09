// use std::cell::RefCell;
// use std::rc::Rc;
use disjoint_set::*;
use std::collections::HashMap;

fn main() {
    // let mut sets = HashMap::new();

    // let h = DSHeader::new_ptr(1);
    // for i in 1..=3 {
    //     sets.insert(i, DSHeader::add(&h, i));
    // }
    // // let a = ItemNode::new_ptr(h.clone(), 1);
    // // let b = a.borrow_mut().set_next(2);
    // // let c = b.borrow_mut().set_next(3);

    // let h2 = DSHeader::new_ptr(2);
    // for i in 4..=5 {
    //     sets.insert(i, DSHeader::add(&h2, i));
    // }
    // let n = ItemNode::new_ptr(h2.clone(), 4);
    // n.borrow_mut().set_next(5);

    // for node in a.borrow_mut().iter_mut() {
    //     println!("bf {}", node.item);
    //     node.item += 1;
    //     println!("af {}\n", node.item);
    // }

    // sets.insert(1, h.clone());
    // sets.insert(2, h2.clone());
    // let mut ds = DisjointSet { sets, cnt: 2 };
    let mut ds = DisjointSet::new();
    for i in 1..=5 {
        ds.make_set(i)
    }

    ds.union(1, 2);
    ds.union(2, 3);

    println!("{}", ds);

    // DSHeader::union(&h, &h2);
    ds.union(4, 5);
    ds.union(1, 4);
    println!("{}", ds);
}
