
pub fn calc_total_sum_recursively(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let result = n + calc_total_sum_recursively(n - 1);

    result
}

pub fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        return m;
    }

    gcd(n, m % n)
}

// フィボナッチ数列の第N項を計算する (0から数えてN項目)
// 極めて効率の悪い実装
// O(((1+√5)/2)^N)
pub fn fibo_recursively(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo_recursively(n-1) + fibo_recursively(n-2),
    }
}

// 再帰的でない、フィボナッチ数列を求める関数
// O(N)
pub fn fibo_repeatedly(n: usize) -> usize {
    let mut vec_fibo = Vec::<usize>::new();

    vec_fibo.push(0); 
    vec_fibo.push(1);

    for i in 2..=n {
        vec_fibo.push(vec_fibo[i-1] + vec_fibo[i-2]);
    }

    vec_fibo[n]
}

// HashMaptによるメモ化を行った、再帰的にフィボナッチ数列を求める関数
// O(N)
use std::collections::HashMap;
fn calc_fibo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    return match memo.get(&n) {
        None => {
            let new_num = calc_fibo(n - 2, memo) + calc_fibo(n - 1, memo);
            memo.insert(n, new_num);
            new_num
        },
        _ => memo[&n]
    }
}

//呼ばれるのはこっち
pub fn fibo_hash(num: u64) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    calc_fibo(num, &mut memo)
}
