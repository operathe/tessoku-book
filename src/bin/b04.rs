use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: i32,
    }

    //文字列に変換
    let n = N.to_string();
    //１０進数に変換
    let n = i32::from_str_radix(&n, 2).unwrap();
    println!("{}", n);
}
