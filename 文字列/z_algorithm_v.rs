//sとsのi番目以降の部分文字列の最長共通接頭辞の長さを求めるアルゴリズム
fn z_algorithm(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0; // 区間の左端
    let mut r = 0; // 区間の右端
    z[0] = n;
    for i in 1..n {
        if i <= r {
            // iがrの範囲内にある場合は、Z配列の値を前回の結果からコピー
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }

        // 文字列の一致部分を拡張する
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }

        // 区間 [l, r] を更新
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }

    z
}
