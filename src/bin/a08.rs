use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        matrix: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut z = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            z[i][j] = z[i][j - 1] + matrix[i - 1][j - 1];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            z[i][j] += z[i - 1][j];
        }
    }

    for &(a, b, c, d) in &abcd {
        let ans = z[c][d] + z[a - 1][b - 1] - z[a - 1][d] - z[c][b - 1];
        println!("{:?}", ans);
    }
}
