use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut v = vec![];
        if start > end {
            v.push(None);
        } else {
            for k in start..=end {
                let left = generate(start, k - 1);
                let right = generate(k + 1, end);

                for l in left.iter() {
                    for r in right.iter() {
                        let root = TreeNode::new_node(k);
                        root.borrow_mut().left = l.clone();
                        root.borrow_mut().right = r.clone();
                        v.push(Some(root));
                    }
                }
            }
        }
        v
    }

    generate(1, n)
}

pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
    let size = tasks.len();
    let mut res = 0;
    let mut process = true;

    while process {
        let mut old = ' ';
        let mut conti = n + 1;
        for c in tasks.iter_mut() {
            println!("{} ", c);
            if c == &' ' {
                continue;
            }
            if c != &old {
                // println!("start: {} {}", old, c);
                res += 1;
                conti -= 1;
                old = *c;
                *c = ' ';
                // println!("end:   {} {}", old, c);
            }
        }

        let mut cnt = 0;
        for c in tasks.iter() {
            if c == &' ' {
                cnt += 1;
            }
        }

        if cnt == size {
            process = false;
        }

        if conti > 0 && process {
            res += conti;
        }
        println!("------ {} -- {} --------", conti, res);
    }

    res
}

fn main() {
    // generate_trees(3);
    let test = vec!['A', 'A', 'A', 'B', 'B', 'B', 'C'];
    let r = least_interval(test, 2);
    println!("{}", r);
}
