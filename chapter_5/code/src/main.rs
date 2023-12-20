mod functions;
fn main() {
    let h: Vec<i32> = vec![2, 9, 4, 5, 1, 6, 10];
    let n = h.len();
    let ans = functions::solv_frog_prob_1(n, h);
    println!("{:?}", ans);
}
