/// 行列累乗による高速フィボナッチ数計算 (0-indexed)
/// fib(0) = 0, fib(1) = 1
fn fibonacci(n: usize, m: usize) -> usize {
    if n == 0 {
        return 0;
    }

    // 基本行列 [[1, 1], [1, 0]]
    let mut a = [[1, 1], [1, 0]];
    matrix_pow(&mut a, n - 1, m);

    a[0][0] // fib(n)
}

/// 2x2行列の累乗（繰り返し二乗法）
fn matrix_pow(mat: &mut [[usize; 2]; 2], mut exp: usize, m: usize) {
    let mut res = [[1, 0], [0, 1]]; // 単位行列

    while exp > 0 {
        if exp % 2 == 1 {
            res = matrix_mul(&res, mat, m);
        }
        *mat = matrix_mul(mat, mat, m);
        exp /= 2;
    }

    *mat = res;
}

/// 2x2行列の積
fn matrix_mul(a: &[[usize; 2]; 2], b: &[[usize; 2]; 2], MOD: usize) -> [[usize; 2]; 2] {
    [
        [
            (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % MOD,
            (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % MOD,
        ],
        [
            (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % MOD,
            (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % MOD,
        ],
    ]
}
