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
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut left = 1;
    let mut right = 10_000_000_000;

    while left < right {
        let mid = (left + right) / 2;
        let mut sum = 0;

        for i in &a {
            sum += mid / i;
        }
        if sum >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", left);
}
