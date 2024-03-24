use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut s = vec![0; d + 1];

    for (l, r) in lr {
        s[l - 1] += 1;
        s[r] -= 1;
    }
    let mut ans = 0;

    for i in &s[..d] {
        ans += i;
        println!("{}", ans);
    }
}
