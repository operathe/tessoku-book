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
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize,usize,usize,usize); n],
    }

    let mut grid = vec![vec![0; w + 2]; h + 2];

    for &(a, b, c, d) in &abcd {
        grid[a][b] += 1;
        grid[a][d + 1] -= 1;
        grid[c + 1][b] -= 1;
        grid[c + 1][d + 1] += 1;
    }

    let mut z = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            z[i][j] = z[i][j - 1] + grid[i][j];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            z[i][j] += z[i - 1][j];
        }
    }

    for row in &z[1..] {
        let ans = row[1..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", ans);
    }
}
