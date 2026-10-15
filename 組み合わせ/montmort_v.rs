//モンモール数 Dnは、「完全順列」の数、すなわち：n 個の要素の順列で、どの要素も 元の位置にない 順列（＝錯順列 / 置換）の個数
// O(n)の計算量で求めることができる。
pub fn montmort(n: usize, modulo: usize) -> Vec<usize> {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    if n >= 1 {
        dp[1] = 0;
    }

    for i in 2..=n {
        dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2]) % modulo;
    }

    dp[1..].to_vec()
}
