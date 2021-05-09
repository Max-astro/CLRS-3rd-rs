use disjoint_set::*;
fn main() {
    let mut set = DSForest::new(5);
    set.print();

    set.union(1, 3);
    set.union(3, 2);
    set.print();

    set.union(4, 0);
    set.print();

    set.union(4, 1);
    set.print();

    let a = set.find_set(0);
    let b = set.find_set(1);
    println!("{} {}", a.borrow().name(), b.borrow().name());
}
