#[allow(unused_imports)]
use either::Either::Right;
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
use rand_core::le;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
#[allow(non_snake_case)]

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }
    let mut right = 0;
    let mut answer = 0;
    for left in 0..(&n - 1) {
        while right + 1 < n && a[right + 1] - a[left] <= k {
            right += 1;
        }
        answer += right - left;
    }
    println!("{}", answer);
}
