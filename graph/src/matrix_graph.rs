pub mod helper {

    pub fn print_matrix(mat: &Vec<Vec<f32>>) {
        for line in mat {
            print!("[ ");
            for i in line {
                if (f32::MAX - *i).abs() <= f32::EPSILON {
                    print! {"INF   "};
                } else {
                    print!("{: <5.1} ", i);
                }
            }
            println!("]");
        }
        println!();
    }

    pub fn parse_graph_to_matrix() -> Vec<Vec<f32>> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // println!("parse_graph_from_stdio:  {}", input);
        let v = input.trim().parse::<usize>().unwrap();
        let mut mat = vec![vec![f32::MAX; v]; v];
        for i in 0..v {
            mat[i][i] = 0.0;
        }

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let e = input.trim().parse::<usize>().unwrap();

        for _ in 0..e {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            // println!("{:?}", input.split_ascii_whitespace());
            // let nums: Vec<&str> = input.trim().split(' ').collect();
            let nums: Vec<&str> = input.split_ascii_whitespace().collect();
            // println!("{:?}", nums);
            // println!("{} {}", nums[0], nums[1]);
            let i = nums[0].trim().parse::<usize>().unwrap();
            let j = nums[1].trim().parse::<usize>().unwrap();
            let weight = nums[2].trim().parse::<f32>().unwrap_or(1.0);

            mat[i][j] = weight;
        }

        mat
    }
}

use helper::*;

pub fn extend_shortest_paths(l: &Vec<Vec<f32>>, w: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n = l.len();
    let mut nextl = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                nextl[i][j] = f32::MAX;
            }
            for k in 0..n {
                nextl[i][j] = f32::min(nextl[i][j], l[i][k] + w[k][j]);
            }
        }
    }

    nextl
}

pub fn show_all_pairs_shortest_paths(w: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n = w.len();
    let mut last_l = w.clone();
    print_matrix(&last_l);

    for _ in 2..n {
        last_l = extend_shortest_paths(&last_l, w);
        print_matrix(&last_l);
    }
    last_l
}

pub fn faster_all_shortest_paths(w: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n = w.len();
    let mut l2m = w.clone();
    print_matrix(&l2m);

    let mut m = 1;
    while m < n - 1 {
        l2m = extend_shortest_paths(&l2m, &l2m);
        print_matrix(&l2m);
        m *= 2;
    }

    l2m
}

pub fn floyd_warshall(w: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n = w.len();
    let mut last_d = w.clone();
    print_matrix(&last_d);

    for k in 0..n {
        // let mut new_d = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                last_d[i][j] = f32::min(last_d[i][j], last_d[i][k] + last_d[k][j]);
            }
        }

        // last_d = new_d;
        print_matrix(&last_d);
    }

    last_d
}
