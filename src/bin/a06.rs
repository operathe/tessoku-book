use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        matrix: [[usize; H]; W],
        Q: usize,
        ABCD: [(usize, usize, usize, usize); Q],
    }
    let mut Z = vec![vec![0; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            Z[i][j] = Z[i][j - 1] + matrix[i - 1][j - 1];
        }
    }
    for j in 1..=W {
        for i in 1..=H {
            Z[i][j] += Z[i - 1][j];
        }
    }

    for &(a, b, c, d) in &ABCD {
        let ans = Z[c][d] + Z[a - 1][b - 1] - Z[a - 1][d] - Z[c][b - 1];
        println!("{}", ans);
    }
}
