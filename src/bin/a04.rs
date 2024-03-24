use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: i32,
    }
    println!("{}", format!("{:010b}", n));
}
