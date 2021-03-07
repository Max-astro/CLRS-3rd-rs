fn matrix_chain_order(p: Vec<(i32, i32)>) -> (Vec<Vec<i32>>, Vec<Vec<usize>>) {
    let n = p.len();
    let mut m = vec![vec![i32::max_value(); n]; n];
    let mut s = vec![vec![0; n]; n];
    for i in 0..n {
        m[i][i] = 0;
    }

    //start from 1
    // for (i, pair) in p.iter().skip(1).enumerate() {

    // }
    for length in 2..n + 1 {
        for i in 0..n - length + 1 {
            let j = i + length - 1;
            // println!("{} {} {}", length, i, j);
            m[i][j] = i32::max_value();
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + pijk(&p, i, k, j);
                // println!("{} {} {} {}", i, j, k, q);
                if j == 5 {
                    // println!("### {} {}", i, q);
                }
                if q < m[i][j] {
                    // println!("{} {} {} {}", n, i, j, q);
                    m[i][j] = q;
                    s[i][j] = k + 1;
                }
            }
        }
    }

    (m, s)
}

fn pijk(v: &Vec<(i32, i32)>, i: usize, j: usize, k: usize) -> i32 {
    v[i].0 * v[j].1 * v[k].1
}

fn print_optimal_parens(s: &Vec<Vec<usize>>, i: usize, j: usize) {
    if i == j {
        print!("A{}", i);
    } else {
        print!("(");
        print_optimal_parens(s, i, s[i][j] - 1);
        print_optimal_parens(s, s[i][j], j);
        print!(")");
    }
}

fn top_down(p: Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    fn lookup_m(p: &[(i32, i32)], m: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if m[i][j] < i32::max_value() {
            return m[i][j];
        }

        // println!("{} {}", i, j);
        if i == j {
            m[i][j] = 0;
        } else {
            // println!("{} {}", i, j);
            for k in i..j {
                // println!("{} {} {}", i, k, j);
                let a = lookup_m(p, m, i, k);
                let b = lookup_m(p, m, k + 1, j);
                if a == i32::max_value() || b == i32::max_value() {
                    continue;
                }

                let q = a + b + p[i].0 * p[k].1 * p[j].1;
                // println!("{} {} {} {}", i, k, j, q);
                if q < m[i][j] {
                    m[i][j] = q;
                }
            }
        }

        m[i][j]
    }

    let n = p.len();
    let mut m = vec![vec![i32::max_value(); n]; n];

    lookup_m(p.as_slice(), &mut m, 0, n - 1);
    m
}

fn main() {
    let p = vec![(30, 35), (35, 15), (15, 5), (5, 10), (10, 20), (20, 25)];

    let (m, s) = matrix_chain_order(p.clone());
    for i in 0..6 {
        print!("{:?}\n", m[i]);
    }

    for i in 0..6 {
        print!("{:?}\n", s[i]);
    }
    print_optimal_parens(&s, 0, 5);

    let m2 = top_down(p);
    for i in 0..6 {
        print!("{:?}\n", m2[i]);
    }

    println!("\n\n");
}
