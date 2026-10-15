pub fn lis<T: Ord>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    // dp[i]: 長さ i+1 のLIS の末尾の値への参照
    let mut dp: Vec<&T> = vec![];
    let mut pos: Vec<usize> = vec![]; // dp[i] に対応するインデックス
    let mut prev = vec![None; n]; // 前のインデックスを記録

    for (i, x) in a.iter().enumerate() {
        let idx = dp.partition_point(|&val| val < x);

        // 広義単調増加（同じ値を許容）にする場合
        //let idx = dp.partition_point(|&val| val <= x);

        if idx == dp.len() {
            dp.push(x);
            pos.push(i);
        } else {
            dp[idx] = x;
            pos[idx] = i;
        }

        if idx > 0 {
            prev[i] = Some(pos[idx - 1]);
        }
    }

    // 復元
    let mut result = vec![];
    let mut current = pos.last().copied(); // LIS の最後のインデックス
    while let Some(i) = current {
        result.push(i);
        current = prev[i];
    }
    result.reverse();
    result
}
