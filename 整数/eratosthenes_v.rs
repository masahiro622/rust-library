/// エラトステネスの篩：n以下の素数を返す O(n log log n)
/// segmented_sieve(l, r): [l, r]の範囲の素数を返す O((r - l + 1) log log r)
/// prime_pi(n): n以下の素数の個数を返す O(n ^ (3/4) log n)
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    primes
}

fn segmented_sieve(l: usize, r: usize) -> Vec<usize> {
    assert!(r >= l);
    let limit = (r as f64).sqrt() as usize + 1;
    let primes = eratosthenes(limit);

    let mut is_prime = vec![true; r - l + 1];

    for &p in &primes {
        // 最初の p の倍数（l 以上）を求める
        let mut start = ((l + p - 1) / p) * p;
        if start < p * p {
            start = p * p;
        }
        for multiple in (start..=r).step_by(p) {
            is_prime[multiple - l] = false;
        }
    }

    if l == 1 {
        is_prime[0] = false;
    }

    (l..=r)
        .zip(is_prime)
        .filter_map(|(n, is_p)| if is_p { Some(n) } else { None })
        .collect()
}

fn prime_pi(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let n_sqrt = n.isqrt();

    let mut larges: Vec<_> = (0..=n_sqrt).collect();
    for i in 1..=n_sqrt {
        larges[i] = n / i - 1;
    }
    let mut smalls: Vec<_> = (0..n / n_sqrt).map(|x| x.saturating_sub(1)).collect();

    for p in 2..=n_sqrt {
        if p < smalls.len() {
            if smalls[p] <= smalls[p - 1] {
                continue;
            }
        } else if larges[n / p] <= smalls[p - 1] {
            continue;
        }
        let pc = smalls[p - 1];
        let q = p * p;
        for i in (1..=n_sqrt).take_while(|&i| n / i >= q) {
            // vi = n / i
            // dp[n / i] -= dp[n / i / p] - pc;
            let ip = i * p;
            let cur = *larges.get(ip).unwrap_or_else(|| &smalls[n / ip]) - pc;
            *larges.get_mut(i).unwrap_or_else(|| &mut smalls[n / i]) -= cur;
        }
        for i in (1..smalls.len()).rev().take_while(|&i| i >= q) {
            // vi = i
            // dp[i] -= dp[i / p] - pc;
            let cur = smalls[i / p] - pc;
            smalls[i] -= cur;
        }
    }
    larges[1]
}
