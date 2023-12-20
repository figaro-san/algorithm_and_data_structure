// code 5.1
// pull-based
pub fn solv_frog_prob_1(n: usize, h: Vec<i32>) -> Vec<usize> {
    let inf: usize = 10000000;
    let mut dp: Vec<usize> = vec![inf; n];
    dp[0] = 0;

    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i] - h[i-1]).abs() as usize;
        } else {
            let cost1 = dp[i-1] + (h[i] - h[i-1]).abs() as usize; // calc cost between i-1 to i
            let cost2 = dp[i-2] + (h[i] - h[i-2]).abs() as usize; // calc cost between i-2 to i
            dp[i] = std::cmp::min(cost1, cost2); // choose smaller one
        }
    }

    dp
}
