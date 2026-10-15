fn main() {
    input! {
        s: Chars, // Nを文字列(文字の配列)として受け取る
    }

    let n = s.len();

    // [TODO] 問題に合わせて状態の最大値を設定する
    // 例: 「各桁の和がK以下」ならK+1など
    let max_state = 100;

    // dp[i][less][state] = 条件を満たすパターン数 (または合計値など)
    let mut dp = vec![vec![vec![0usize; max_state]; 2]; n + 1];

    // [TODO] 初期条件の設定
    dp[0][0][0] = 1; // 0桁目、Nと一致(less=0)、状態0 のパターン数が1通り

    for i in 0..n {
        let max_d = s[i].to_digit(10).unwrap() as usize;

        for less in 0..2 {
            for state in 0..max_state {
                // 枝刈り: パターン数が0なら計算しない（高速化）
                if dp[i][less][state] == 0 {
                    continue;
                }

                // 次の桁として置ける最大の数
                let lim = if less == 1 { 9 } else { max_d };

                for d in 0..=lim {
                    // 1. 次の less フラグの計算
                    let next_less = if less == 1 || d < max_d { 1 } else { 0 };

                    // 2. [TODO] 次の状態の計算
                    // 例: 各桁の和なら next_state = state + d;
                    let next_state = state;

                    // 状態が範囲外になる場合は遷移しない
                    if next_state >= max_state {
                        continue;
                    }

                    // 3. [TODO] 遷移の計算（基本は足し算）
                    dp[i + 1][next_less][next_state] += dp[i][less][state];
                }
            }
        }
    }

    // [TODO] 答えの集計
    let mut ans = 0;
    for less in 0..2 {
        // 例: state が特定の条件を満たすものを足し合わせる
        // ans += dp[n][less][target_state];
    }

    println!("{}", ans);
}
