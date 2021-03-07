pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();

    let n1 = s1.len();
    let n2 = s2.len();

    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

    for i in 1..=n1 {
        for j in 1..=n2 {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[n1][n2]
}

pub fn lcs_better_space(text1: String, text2: String) -> i32 {
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();

    let n1 = s1.len();
    let n2 = s2.len();

    let mut dp = vec![0; n2 + 1];

    for i in 1..=n1 {
        let mut cur = vec![0; n2 + 1];
        for j in 1..=n2 {
            if s1[i - 1] == s2[j - 1] {
                cur[j] = dp[j - 1] + 1;
            } else {
                // println!("i: {}, j: {}\n{:?}\n{:?}", i, j, dp, cur);
                cur[j] = std::cmp::max(dp[j], cur[j - 1]);
            }

            println!("\n{:?}\n{:?}", dp, cur);
        }
        dp = cur;
    }

    dp[n2]
}

// pub fn num_bstrees(n: i32) -> i32 {
//     fn lookup(dp :&Vec<i32>, i:usize, j:usize) -> i32 {

//     }
// }
pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1; n + 2];
    dp[2] = 2;

    for i in 3..=n {
        let mut sum = 0;
        for j in 1..=i {
            let tmp = dp[j - 1] * dp[i - j];
            sum += tmp;

            // println!(
            //     "{} {} {} {}",
            //     j - 1,
            //     i - j,
            //     tmp,
            //     sum
            // );
        }
        dp[i] = sum;
    }
    println!("{:?}", dp);
    dp[n]
}

pub fn num_bstress_topdown(n: i32) -> i32 {
    fn lookup(n: usize, dp: &mut Vec<i32>) -> i32 {
        if dp[n] != 0 {
            dp[n]
        } else {
            let mut sum = 0;
            for i in 1..=n {
                sum += lookup(i - 1, dp) * lookup(n - i, dp);
            }
            dp[n] = sum;
            sum
        }
    }

    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 1;

    lookup(n, &mut dp)
}

fn main() {
    // let text1 = "abcde".to_string();
    // let text2 = "ace".to_string();
    // assert_eq!(lcs_better_space(text1, text2), 3);
    num_trees(4);
    println!("\n\n###################\n\n");
    num_trees(5);
    println!("\n\n###################\n\n");
    num_trees(6);
}
