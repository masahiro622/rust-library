/// 素因数分解を行い、オイラーのφ関数を計算する関数
pub fn phi(mut n: usize) -> usize {
    let mut result = n;
    let mut p = 2;

    // 素因数分解を行い、オイラーのφ関数を求める
    while p * p <= n {
        if n % p == 0 {
            // pが素因数ならば、nをpで割り続ける
            while n % p == 0 {
                n /= p;
            }
            // φ(n)の更新
            result -= result / p;
        }
        p += 1;
    }

    // 最後にnが1より大きければ、それは素数なので結果に反映
    if n > 1 {
        result -= result / n;
    }

    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// mod M における a^b
fn modpow(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1usize;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            res = res.wrapping_mul(a) % m;
        }
        a = a.wrapping_mul(a) % m;
        b >>= 1;
    }
    res
}

// A↑↑B mod M
fn tetration(a: usize, b: usize, m: usize) -> usize {
    if m == 1 {
        return 0;
    }
    if a == 0 {
        return 1 ^ (b & 1);
    }
    if b == 0 {
        return 1;
    }
    if b == 1 {
        return a % m;
    }
    if b == 2 {
        return modpow(a, a, m);
    }

    let phi_m = phi(m);
    let mut exp = tetration(a, b - 1, phi_m);
    if exp == 0 {
        exp += phi_m;
    }
    modpow(a, exp, m)
}
