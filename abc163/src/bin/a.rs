use proconio::input;

fn main() {
    input! {
        r: f64,
    }
    let ans = r*2.*std::f64::consts::PI;
    println!("{}", ans);
}