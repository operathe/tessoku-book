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
#[allow(unused_imports)]
use std::ops::Bound;
#[allow(unused_imports)]
use std::vec;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
        b:[usize; n],
        c:[usize; n],
        d:[usize; n],
    }
    //a,b を足した配列を作る
    let mut ab = iproduct!(a.iter(), b.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<usize>>();
    // c,d を足した配列を作る
    let mut cd = iproduct!(c.iter(), d.iter())
        .map(|(c, d)| c + d)
        .collect::<Vec<usize>>();
    // ab と cd をソートする
    ab.sort();
    cd.sort();
    // ab と cd を足した数が k 以下になる組み合わせがあるか半分全列挙で判定する
    let mut answer = "No";
    for &a in &ab {
        // k -a がcdの中にあるか判別する
        if cd.lower_bound(&(k - a)) < cd.len() && cd[cd.lower_bound(&(k - a))] == k - a {
            answer = "Yes";
            break;
        }
    }
    println!("{}", answer);

    // ab と cd を足した数が k 以下になる組み合わせがあるかしゃくとり法を使ってyes no判定する
    // let mut right = cd.len();
    // let mut answer = "No";
    // for left in 0..n {
    //     while right > 0 && ab[left] + cd[right - 1] > k {
    //         right -= 1;
    //     }
    //     if right > 0 && ab[left] + cd[right - 1] == k {
    //         answer = "Yes";
    //         break;
    //     }
    // }
    // println!("{}", answer);
}
