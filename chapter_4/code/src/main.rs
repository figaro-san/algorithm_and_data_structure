#![allow(unused)]

use std::vec;
mod functions;

fn main() {
    let w = 10; // target
    let i = 5; //要素数
    let mut a = vec![1, 2, 3, 4, 5];
    let ans = functions::solve_subset_sum_problem(i, w, &mut a);
    println!("{ans}");
}
