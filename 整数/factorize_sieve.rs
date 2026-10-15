fn sieve(n: usize) -> Vec<usize> {
    let mut spf = vec![0; n + 1];

    // 各数の最小素因数で初期化
    for i in 1..=n {
        spf[i] = i;
    }

    // 2から√nまでの各素数について
    for i in 2..=((n as f64).sqrt() as usize) {
        if spf[i] == i {
            // i が素数の場合
            // i の倍数で、まだ最小素因数が設定されていないものを更新
            for j in (i * i..=n).step_by(i) {
                if spf[j] == j {
                    // まだ最小素因数が自分自身の場合のみ更新
                    spf[j] = i;
                }
            }
        }
    }

    spf
}

fn factor(n: usize, spf: &[usize]) -> rustc_hash::FxHashMap<usize, usize> {
    let mut res = rustc_hash::FxHashMap::default();
    let mut x = n;
    while x != 1 {
        *res.entry(spf[x]).or_insert(0) += 1;
        x /= spf[x];
    }
    res
}
