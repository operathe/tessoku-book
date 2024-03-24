use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut ans: i32 = 0;
    for a in 1..=n {
        for b in 1..=n {
            if k - a - b >= 1 && k - a - b <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
