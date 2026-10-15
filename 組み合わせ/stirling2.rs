/*n 個の異なる要素を ちょうど k 個の非空集合に分割する方法の数
例：S(3, 2) = 3（{1,2},{3} など） */

pub fn stirling2(n: usize, k: usize, modulo: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        for j in 1..=k.min(i) {
            dp[i][j] = (j * dp[i - 1][j] + dp[i - 1][j - 1]) % modulo;
        }
    }

    dp[n][k]
}
