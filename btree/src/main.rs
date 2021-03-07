mod node;
use node::*;

#[macro_export]
macro_rules! insert_print {
    ( $a:expr, $key:expr  ) => {
        $a.insert($key as u8 - '@' as u8, '.');
        println!(
            "\nAfter insert '{}', \nroot: {}",
            $key,
            $a.get_root().borrow()
        );
        for (idx, node) in $a.get_root().borrow().childs.iter().enumerate() {
            println!("child_{}: {}", idx, node.borrow());
            if node.borrow().get_n() != 0 {
                for (i, child) in node.borrow().childs.iter().enumerate() {
                    println!("child_{} sub_{}: {}", idx, i, child.borrow());
                }
            }
        }
    };

    ( $a:expr, $( $key:expr ),* ) => {
        {
            $($a.insert($key as u8 - '@' as u8, '.');)*
            println!("Root: {}", $a.get_root().borrow());
            for (idx, node) in $a.get_root().borrow().childs.iter().enumerate() {
                println!("child_{}: {}", idx, node.borrow());
                if node.borrow().get_n() != 0 {
                    for (i, child) in node.borrow().childs.iter().enumerate() {
                        println!("child_{} sub_{}: {}", idx, i, child.borrow());
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! print_tree {
    ( $a:expr ) => {
        // println!("[{}] elems. Root: {}", $a.size(), $a.get_root().borrow());
        // for (idx, node) in $a.get_root().borrow().childs.iter().enumerate() {
        //     println!("child_{}: {}", idx, node.borrow());
        //     if node.borrow().get_n() != 0 {
        //         for (i, child) in node.borrow().childs.iter().enumerate() {
        //             println!("child_{} sub_{}: {}", idx, i, child.borrow());
        //         }
        //     }
        // }

        let n = $a.size();
        println!("Size: {}", n);
        let mut que = std::collections::VecDeque::new();
        que.push_back(($a.get_root(), false));
        que.push_back(($a.get_root(), true));
        let mut idx = 0;
        while let Some((node, ln)) = que.pop_front() {
            if !ln {
                print!("#{}#{} -> ", idx, node.borrow());
                idx += 1;

                if !node.borrow().isleaf() {
                    for tmp in node.borrow().childs.iter() {
                        que.push_back((tmp.clone(), false));
                    }
                }
            } else if !que.is_empty() {
                idx = 0;
                que.push_back((node.clone(), true));
                println!("\n-------------------------------------------");
            }
        }
        println!("\n\n");
    };
}

#[macro_export]
macro_rules! del_char {
    ( $a:expr, $key:expr ) => {
        $a.delete(&($key as u8 - '@' as u8));
    };
}

fn main() {
    let mut a: Btree<u8, u8> = Btree::new(2);

    // /* Split test */
    // {
    //     a.fill_insert();
    //     {
    //         let n1 = Btree::get_nchild(a.get_root(), 0);
    //         println!("child_0 {}", n1.borrow());
    //     }

    //     println!("root {}", a.get_root().borrow());
    //     Btree::split_child(a.get_root(), 0);
    //     println!("{}", a.get_root().borrow());
    //     println!("E node: {}", Btree::get_nchild(a.get_root(), 0).borrow());
    //     println!("K node: {}", Btree::get_nchild(a.get_root(), 1).borrow());
    // }

    // /* Insert test 1*/
    // {
    //     // a.get_root().borrow_mut().fill();
    //     println!("root: {}", a.get_root().borrow());

    //     a.insert('Z', 'Z');
    //     println!("\nAfter insert 'Z', \nroot: {}", a.get_root().borrow());
    //     for (idx, node) in a.get_root().borrow().childs.iter().enumerate() {
    //         println!("child_{}: {}", idx, node.borrow());
    //     }

    //     insert_print!(a, 'A');
    //     insert_print!(a, 'C');
    //     insert_print!(a, 'E');
    //     for i in 'a'..'g' {
    //         insert_print!(a, i);
    //     }

    //     let min_k = Btree::get_min_item(a.get_root());
    //     println!("min key: {}", min_k.unwrap());
    // }

    /* Book case: Insert test */
    // insert_print!(
    //     a, 'F', 'S', 'Q', 'K', 'C', 'L', 'H', 'T', 'V', 'W', 'M', 'R', 'N', 'P', 'A', 'B', 'X',
    //     'Y', 'D', 'Z', 'E'
    // );

    // /* Test case delete */
    // let ks = vec!['F', 'S', 'Q', 'K', 'C', 'L', 'H', '@'];
    // for c in ks {
    //     a.insert(c as u8 - '@' as u8, c as u8);
    // }

    // a.delete(&(6u8));
    // println!("\nAfter delete '6'\n");
    // print_tree!(a);
    // a.delete(&(0u8));
    // println!("\nAfter delete '0'\n");
    // print_tree!(a);
    // a.delete(&(8u8));
    // println!("\nAfter delete '8'\n");
    // print_tree!(a);
    // a.delete(&(19u8));
    // println!("\nAfter delete '19'\n");
    // print_tree!(a);

    // a.delete(&(3u8));
    // println!("\nAfter delete '3'\n");
    // print_tree!(a);
    // a.delete(&(11u8));
    // println!("\nAfter delete '11'\n");
    // print_tree!(a);

    // a.delete(&(12u8));
    // println!("\nAfter delete '12'\n");
    // print_tree!(a);
    // a.delete(&(17u8));
    // println!("\nAfter delete '17'\n");
    // print_tree!(a);
    // /* delete test end */
    // /* case many items 1*/
    // for k in 0..=40 {
    //     a.insert(k, k);
    // }
    // print_tree!(a);

    // for k in (0..=40).rev() {
    //     let d = a.delete(&k);
    //     println!("\n\nAfter delete '{:?}'\n", d);
    //     print_tree!(a);
    // }

    /* case many items 2*/
    let mut a: Btree<i32, i32> = Btree::new(10);
    for k in -2550..=2550 {
        a.insert(k, k);
    }
    print_tree!(a);

    for k in (-2550..=2550).rev() {
        a.delete(&k);
    }
    print_tree!(a);
    // println!("######### After DELETE #########");
    // print_tree!(a);
}
