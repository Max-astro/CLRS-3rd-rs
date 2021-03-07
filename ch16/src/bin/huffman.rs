use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Debug, Clone, Eq)]
struct Node {
    freq: isize,
    c: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn merge(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        let freq = left.as_ref().map(|node| node.freq).unwrap()
            + right.as_ref().map(|node| node.freq).unwrap();
        Box::new(Node {
            freq,
            c: None,
            left: left,
            right: right,
        })
    }

    fn new(c: char, freq: isize) -> Box<Node> {
        Box::new(Node {
            freq,
            c: Some(c),
            left: None,
            right: None,
        })
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.freq.cmp(&other.freq)
        let res = self.freq - other.freq;
        if res > 0 {
            Ordering::Less
        } else if res < 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

fn huffman_tree_encode(cases: Vec<(char, isize)>) -> Box<Node> {
    let mut heap = BinaryHeap::new();
    for pair in cases {
        heap.push(Node::new(pair.0, pair.1));
    }

    while heap.len() > 1 {
        let new = Node::merge(heap.pop(), heap.pop());
        println!("{:?} {}", new.c, new.freq);
        heap.push(new);
    }

    heap.pop().unwrap()
}

fn huffman_tree_decode(s: String, root: &Box<Node>) -> String {
    let mut res = String::new();
    let mut node = root;
    for c in s.chars() {
        match c {
            '0' => node = node.left.as_ref().unwrap(),
            '1' => node = node.right.as_ref().unwrap(),
            _ => panic!("Invaild input!"),
        }

        if let Some(alp) = node.c.clone() {
            res.push(alp);
            node = root;
        };
    }

    res
}

fn main() {
    let cases = vec![
        ('a', 45),
        ('b', 13),
        ('c', 12),
        ('d', 16),
        ('e', 9),
        ('f', 5),
    ];

    let root = huffman_tree_encode(cases);
    // println!("{:?}", root);
    let res = huffman_tree_decode("001011101".to_string(), &root);
    println!("{}", res);
}
