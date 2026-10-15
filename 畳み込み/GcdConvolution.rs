// require "../modint.rs" , prime_list

fn gcd_convolution(a: &mut Vec<Mint>, b: &mut [Mint]) -> Vec<Mint> {
    let n = a.len() - 1;
    debug_assert_eq!(a.len(), b.len());

    let primes = prime_list(n);

    // 倍数ゼータ変換 (Multiple Zeta Transform)
    // A_k = sum_{k|i} A_i

    for &p in &primes {
        for i in (1..=n / p).rev() {
            let ta = a[i * p];
            let tb = b[i * p];
            a[i] += ta;
            b[i] += tb;
        }
    }

    // 各点積
    for i in 1..=n {
        a[i] *= b[i];
    }

    // 倍数メビウス変換 (Inverse Multiple Zeta Transform)
    for &p in &primes {
        for i in 1..=n / p {
            let tc = a[i * p];
            a[i] -= tc;
        }
    }

    std::mem::take(a)
}
