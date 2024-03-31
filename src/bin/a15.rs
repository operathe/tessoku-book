#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut B: Vec<_> = A.clone();
    B.sort();
    B.dedup();

    let mut ans = vec![0; N];

    for (i, a) in A.iter().enumerate() {
        ans[i] = B.lower_bound(a) + 1;
    }
    println!("{}", ans.iter().join(" "));
}
