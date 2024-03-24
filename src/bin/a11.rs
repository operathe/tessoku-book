use proconio::{fastout, input};

fn binary_search(v: &[usize], x: usize) -> usize {
    let mut left = 0;
    let mut right = v.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if x <= v[mid] {
            right = mid - 1;
        } else if x == v[mid] {
            return mid;
        } else {
            left = &mid + 1;
        }
    }
    left
}
#[fastout]

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }
    let ans = binary_search(&a, x);
    println!("{}", ans + 1);
}
