extern crate rand;

use rand::prelude::*;

fn partition(nums: &mut [i32], lo: usize, hi: usize) -> usize {
    let mut rng = thread_rng();
    nums.swap(hi - 1, rng.gen_range(lo..=hi - 1));
    let x = nums[hi - 1];

    let mut i = lo;
    for j in lo..(hi - 1) {
        if nums[j] <= x {
            nums.swap(i as usize, j);
            i += 1;
        }
    }

    nums.swap(i, hi - 1);
    // println!("### {} {} {} ###  | {:?}", lo, i, hi, nums);
    i
}

fn quick_sort(nums: &mut [i32], lo: usize, hi: usize) {
    if lo + 1 >= hi {
        return;
    }

    let mid = partition(nums, lo, hi);
    if mid > 0 {
        quick_sort(nums, lo, mid);
    }
    if mid + 1 < hi {
        quick_sort(nums, mid + 1, hi);
    }
}

fn find_k_small(nums: &mut [i32], lo: usize, hi: usize, smallest: usize) -> i32 {
    if lo == hi {
        return nums[lo];
    }

    let par = partition(nums, lo, hi);
    if par == smallest {
        return nums[par];
    } else if smallest < par {
        return find_k_small(nums, lo, par, smallest);
    } else {
        return find_k_small(nums, par + 1, hi, smallest - par);
    }
}

fn median(nums: &mut [i32]) -> i32 {
    let len = nums.len();
    if len % 2 == 1 {
        find_k_small(nums, 0, len, len / 2)
    } else {
        let n1 = find_k_small(nums, 0, len, len / 2);
        let n2 = find_k_small(nums, 0, len, len / 2 - 1);
        // println!("{} {}", n1, n2);
        (n1 + n2) / 2
    }
}

fn insert_sort(nums: &mut [i32], lo: usize, hi: usize) {
    for j in lo + 1..hi {
        let key = nums[j];
        let mut i = j as i32 - 1;
        while i >= lo as i32 && nums[i as usize] > key {
            nums[i as usize + 1] = nums[i as usize];
            i -= 1;
        }
        // println!("i: {}, j: {}, | {:?}", i, j, nums);
        nums[i as usize + 1] = key;
    }
}

fn quick_median(nums: &mut [i32]) -> i32 {
    let len = nums.len();
    let mut start = 0usize;
    let mut mids = vec![];
    while start < len {
        if start + 5 <= len {
            insert_sort(nums, start, start + 5);
            mids.push(nums[start + 2]);
        } else {
            insert_sort(nums, start, len);
            let mid;
            if len % 2 == 1 {
                mid = nums[(len - start) / 2];
            } else {
                mid = (nums[(len - start) / 2] + nums[(len - start) / 2 - 1]) / 2;
            }
            mids.push(mid);
        }
        start += 5;
    }
    println!("{:?}", nums);
    println!("{:?}", mids);
    median(mids.as_mut_slice())
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m]; 26];

    let vs = s.as_bytes();
    let ts = t.as_bytes();

    for i in (0..m).rev() {
        for j in 0..26 {
            if ts[i] == j + 'a' as u8 {
                dp[i][j as usize] = i;
            }
            dp[i][j as usize] = dp[i + 1][j as usize];
        }
    }

    let mut index = 0;
    for c in ts {
        let chr = (c - 'a' as u8) as usize;
        if dp[index][chr] == m {
            return false;
        }
        index = dp[index][chr] + 1;
    }

    true
}

fn main() {
    let mut rng = thread_rng();

    let mut data = vec![];
    for _ in 0..10 {
        let a: i32 = rng.gen_range(-100..=100);
        data.push(a);
    }
    println!("{:?}", data);
    let hi = data.len();

    // quick_sort(data.as_mut_slice(), 0, hi);
    // let par = find_k_small(data.as_mut_slice(), 0, hi, 9);
    // println!("{}", par);

    let mid = median(data.as_mut_slice());
    let qmid = quick_median(data.as_mut_slice());
    println!("{} {}", mid, qmid);

    for i in (0..10).rev() {
        print!("{} ", i);
    }
}
