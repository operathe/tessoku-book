#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q]
    }

    // 累積和を計算
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + if a[i] == 1 { 1 } else { -1 };
    }

    // クエリを処理
    for (l, r) in lr {
        let count = sum[r] - sum[l - 1];
        if count < 0 {
            println!("lose");
        } else if count > 0 {
            println!("win");
        } else {
            println!("draw");
        }
    }
}
