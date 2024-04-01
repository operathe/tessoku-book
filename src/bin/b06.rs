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
    //累積和で解く
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + if a[i] == 1 { 1 } else { -1 };
    }
    for (l, r) in lr {
        let count = sum[r] - sum[l - 1];
        match count.cmp(&0) {
            std::cmp::Ordering::Less => println!("lose"),
            std::cmp::Ordering::Greater => println!("win"),
            std::cmp::Ordering::Equal => println!("draw"),
        }
    }
}
