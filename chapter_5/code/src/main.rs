mod functions;
fn main() {
    let n = 7;
    let h: Vec<i32> = vec![2, 9, 4, 5, 1, 6, 10];
    let ans = functions::solv_frog_prob(n, h);
    println!("{:?}", ans);
}
