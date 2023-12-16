
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

// メモ化を行った、再帰的にフィボナッチ数列を求める関数
// O(N)
pub fn fibo_memo_recursively(n: usize) {
}
