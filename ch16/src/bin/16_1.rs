// use std::VecDeque;

pub fn recursive_activity_selector(mut list: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    fn helper(list: &Vec<(usize, usize)>, k: usize, n: usize) -> Vec<(usize, usize)> {
        let mut m = k + 1;
        let mut res = vec![];
        // find the first start time > endtime[k] activity
        while m < n && list[m].0 < list[k].1 {
            m += 1;
        }
        if m < n {
            res.push(list[m]);
            res.append(&mut helper(list, m, n));
        }
        // println!("{:?}", list);
        // println!("k:{} m:{} | {:?}", k, m, res);
        res
    }

    list.push((0, 0));
    list.sort_by(|a, b| a.1.cmp(&b.1));

    helper(&list, 0, list.len())
}

pub fn greedy_activity_selector(mut list: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    list.push((0, 0));
    list.sort_by(|a, b| a.1.cmp(&b.1));

    let mut res = vec![];
    let mut k = 0;

    for m in 1..list.len() {
        if list[m].0 >= list[k].1 {
            res.push(list[m]);
            k = m;
        }
    }

    res
}

fn main() {
    let a = vec![
        (1, 4),
        (3, 5),
        (0, 6),
        (5, 7),
        (3, 9),
        (5, 9),
        (6, 10),
        (8, 11),
        (8, 12),
        (2, 14),
        (12, 16),
    ];

    let res = greedy_activity_selector(a);
    println!("{:?}", res);
}
