use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        a: [i32; m],
    }
    let sum: i32 = a.iter().sum();
    let days = n-sum;
    if days < 0 {
        println!("-1");
    } else {
        println!("{}", days);
    }
}
