#![allow(unused)]

use std::vec;
mod functions;

fn main() {
    let ans = functions::fibo_repeatedly(10);
    println!("{ans}");
}
