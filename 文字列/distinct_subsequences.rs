// （連続するとは限らない）部分列であって、重複しないものの個数を数える
//　空文字列も含む
use std::collections::HashMap;
use std::hash::Hash;
fn count_distinct_subsequences<T: Eq + Hash + Copy>(s: &[T]) -> Mint {
    let n = s.len();

    let mut dp = vec![Mint::raw(0); n + 1];
    dp[0] = Mint::raw(1); // 空文字列

    let mut last = HashMap::new();

    for i in 0..n {
        dp[i + 1] = dp[i] * 2;

        if let Some(&j) = last.get(&s[i]) {
            dp[i + 1] = dp[i + 1] - dp[j];
        }

        last.insert(s[i], i);
    }

    dp[n]
}
