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
        a: usize, b: usize,
    }
    let mut ans = 0;
    for i in a..=b {
        if 100 % i == 0 {
            ans += 1;
            break;
        }
    }
    if ans == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
