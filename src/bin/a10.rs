#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections;
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
            n: usize,
            a: [usize; n],
            d: usize,
            lr: [(usize, usize); d]
    }
    let mut p = vec![0; n + 1];
    let mut q = vec![0; n + 1];

    for i in 1..=n {
        p[i] = cmp::max(p[i - 1], a[i - 1]);
    }
    for j in (1..=n).rev() {
        q[j - 1] = cmp::max(q[j], a[j - 1]);
    }
    for &(l, r) in &lr {
        let ans = cmp::max(p[l - 1], q[r]);
        println!("{}", ans);
    }
}
