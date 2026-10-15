use std::collections::HashMap;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// 拡張ユークリッドの互除法: ax + by = gcd(a, b)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, y, x) = ext_gcd(b, a % b);
        (d, x, y - (a / b) * x)
    }
}

// 逆元計算: a^{-1} mod m
fn mod_inv(a: i64, m: i64) -> i64 {
    let (_, x, _) = ext_gcd(a, m);
    (x % m + m) % m
}

fn discrete_logarithm(x: i64, mut y: i64, mut m: i64) -> Option<i64> {
    // コーナーケース: M=1 なら 0 ≡ 0 なので K=0 (最小)
    if m == 1 {
        return Some(0);
    }
    // コーナーケース: Y=1 なら X^0 = 1 なので K=0
    if y == 1 {
        return Some(0);
    }
    // コーナーケース: Y=0 かつ X=0 なら 0^1=0 なので K=1 (0^0=1なのでK=0は不可)
    if x == 0 && y == 0 {
        return Some(1);
    }

    // Step 1: GCD Reduction (互いに素になるまで係数をくくりだす)
    // 式: coeff * X^k ≡ Y (mod m)
    let mut coeff = 1;
    let mut add_k: i64 = 0; // 削減した回数（Kの加算分）

    // 最大でも log(M) 回程度で収束、あるいは互いに素になる
    loop {
        // 現在の状態で解が見つかるか確認 (小さいKのチェック)
        if coeff == y {
            return Some(add_k);
        }

        let g = gcd(x, m);
        if g == 1 {
            break; // 互いに素になったらBSGSへ
        }

        if y % g != 0 {
            return None; // Yがgで割り切れないなら解なし
        }

        // 変形: (coeff * X) * X^{k-1} ≡ Y (mod M)
        // 両辺と法を g で割る
        y /= g;
        m /= g;
        coeff = (coeff * (x / g)) % m;
        add_k += 1;
    }

    // Step 2: Baby-step Giant-step
    // 今、解くべき式は: coeff * X^k ≡ Y (mod m)  (ただし gcd(X, m) == 1)
    // ここでの k は、元の K から add_k を引いたもの。

    // Baby-step: X^j を計算して保存
    let m_sqrt = (m as f64).sqrt().ceil() as i64;

    let mut baby_steps = HashMap::new();
    let mut curr = coeff;

    // j: 0 .. m_sqrt-1
    for j in 0..m_sqrt {
        baby_steps.entry(curr).or_insert(j);
        curr = (curr * x) % m;
    }

    // Giant-step: Y * (X^{-m_sqrt})^i を計算して照合
    // X^m_sqrt を計算
    let mut giant_step_val = 1;
    for _ in 0..m_sqrt {
        giant_step_val = (giant_step_val * x) % m;
    }

    // X^m_sqrt の逆元
    let giant_step_inv = mod_inv(giant_step_val, m);

    let mut target = y;

    // i: 0 .. m_sqrt
    for i in 0..=m_sqrt {
        if let Some(&j) = baby_steps.get(&target) {
            // 解が見つかった: K_residual = i * m_sqrt + j
            let k_res = i * m_sqrt + j;
            return Some(add_k + k_res);
        }
        target = (target * giant_step_inv) % m;
    }

    None
}
