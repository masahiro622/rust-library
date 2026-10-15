//　無平方数を数える関数　(O(N^(2/5)))

use std::collections::HashMap;
use std::io;

// 高速化された無平方数カウンター
fn square_free(n: u64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // Imax = N^(1/3) の近似値を計算
    let imax = ((n as f64).cbrt() as u64).max(1);
    let d = integer_sqrt(n / imax);

    // メビウス関数をd以下で計算
    let mobius = compute_mobius(d);

    // S1を計算: Σ μ(i) * (N / i²) for i = 1 to D
    let mut s1 = 0i64;
    for i in 1..=d {
        s1 += mobius[i as usize] * (n / (i * i)) as i64;
    }

    // M(d) = Σ μ(k) for k = 1 to d の累積和を計算
    let mut m_list = vec![0i64; (d + 1) as usize];
    let mut m_sum = 0i64;
    for i in 1..=d {
        m_sum += mobius[i as usize];
        m_list[i as usize] = m_sum;
    }

    // M(√(n/i)) を i = imax-1, ..., 1 について計算
    let mut mxi_cache: HashMap<u64, i64> = HashMap::new();
    let mut mxi_sum = 0i64;

    for i in (1..imax).rev() {
        let xi = integer_sqrt(n / i);
        let mxi = compute_m_xi(xi, d, &m_list, &mut mxi_cache, n);
        mxi_sum += mxi;
    }

    // S2を計算
    let s2 = mxi_sum - (imax as i64 - 1) * m_list[d as usize];

    s1 + s2
}

// M(xi) = Σ μ(k) for k = 1 to xi を効率的に計算
fn compute_m_xi(xi: u64, d: u64, m_list: &[i64], cache: &mut HashMap<u64, i64>, n: u64) -> i64 {
    if xi <= d {
        return m_list[xi as usize];
    }

    if let Some(&cached) = cache.get(&xi) {
        return cached;
    }

    let mut mxi = 1i64; // M(xi) = 1 + Σ μ(k) for k = 2 to xi
    let sqd = integer_sqrt(xi);

    // 調和級数を使った高速化部分
    // xi/j の値が同じになる区間をまとめて処理
    for j in 1..=xi / (sqd + 1) {
        if j <= d {
            mxi -= (xi / j - xi / (j + 1)) as i64 * m_list[j as usize];
        }
    }

    // 大きな除数について個別に処理
    for j in 2..=sqd {
        let quotient = xi / j;
        if quotient <= d {
            mxi -= m_list[quotient as usize];
        } else {
            mxi -= compute_m_xi(quotient, d, m_list, cache, n);
        }
    }

    cache.insert(xi, mxi);
    mxi
}

// 整数平方根を計算
fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut x = n;
    let mut y = (n + 1) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}

// メビウス関数をn以下で計算
fn compute_mobius(n: u64) -> Vec<i64> {
    let mut mobius = vec![0i64; (n + 1) as usize];
    let mut is_prime = vec![true; (n + 1) as usize];

    if n >= 1 {
        mobius[1] = 1;
    }

    for i in 2..=n {
        if is_prime[i as usize] {
            // iが素数の場合
            mobius[i as usize] = -1;

            // iの倍数を処理
            let mut j = 2 * i;
            while j <= n {
                is_prime[j as usize] = false;

                // メビウス関数の更新
                if (j / i) % i == 0 {
                    // i²で割り切れる場合
                    mobius[j as usize] = 0;
                } else {
                    // iで一度だけ割り切れる場合
                    mobius[j as usize] = -mobius[(j / i) as usize];
                }

                j += i;
            }
        }
    }

    mobius
}
