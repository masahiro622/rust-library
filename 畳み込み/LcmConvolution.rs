// requires: prime_list , modint

fn lcm_convolution(a: &mut Vec<Mint>, b: &mut [Mint]) -> Vec<Mint> {
    let n = a.len() - 1;
    let primes = prime_list(n);

    // 倍数ゼータ変換 (Multiple Zeta Transform)
    // A_k = sum_{k|i} A_i

    for &p in &primes {
        for i in 1..=n / p {
            let ta = a[i];
            let tb = b[i];
            a[i * p] += ta;
            b[i * p] += tb;
        }
    }

    // 各点積
    for i in 1..=n {
        a[i] *= b[i];
    }

    // 倍数メビウス変換 (Inverse Multiple Zeta Transform)
    for &p in &primes {
        for i in (1..=n / p).rev() {
            let tc = a[i];
            a[i * p] -= tc;
        }
    }

    std::mem::take(a)
}
