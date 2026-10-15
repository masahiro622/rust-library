// nCm を計算するライブラリ
// n は 5*10^3 程度まで
// k は n 以下

/// nCk を 0 <= k <= n <= N で全て前計算する
fn binom_table(n: usize, modulo: usize) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1 % modulo;
        c[i][i] = 1 % modulo;
        for j in 1..i {
            c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % modulo;
        }
    }
    c
}
