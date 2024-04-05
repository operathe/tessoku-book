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
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        t: usize, n: usize,
        lr: [(usize, usize); n],
    }
    let mut s = vec![0; t + 1];
    for &(l, r) in &lr {
        s[l] += 1;
        s[r] -= 1;
    }
    let mut ans = 0;
    for i in &s[..t] {
        ans += i;
        println!("{}", ans);
    }
}
