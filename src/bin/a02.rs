use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    for i in a {
        if i == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
