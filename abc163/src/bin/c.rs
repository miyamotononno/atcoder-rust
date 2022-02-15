use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n-1],
    }
    let mut ans = vec![0; n as usize];
    for v in a.iter() {
        ans[(v-1) as usize]+=1;
    }
    for r in ans.iter() {
        println!("{}", r);
    }
}
