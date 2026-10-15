fn prime_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![false; n + 1];

    // 2と3は特別扱い
    if n >= 2 {
        is_prime[2] = true;
    }
    if n >= 3 {
        is_prime[3] = true;
    }

    // --- ステップ1: ふるいの処理（トグル操作） ---
    let mut x = 1;
    while x * x <= n {
        let mut y = 1;
        while y * y <= n {
            let xx = x * x;
            let yy = y * y;

            // 条件1: n = 4x^2 + y^2
            // n % 12 == 1 または 5 の場合、判定を反転
            let n1 = 4 * xx + yy;
            if n1 <= n && (n1 % 12 == 1 || n1 % 12 == 5) {
                is_prime[n1] = !is_prime[n1];
            }

            // 条件2: n = 3x^2 + y^2
            // n % 12 == 7 の場合、判定を反転
            let n2 = 3 * xx + yy;
            if n2 <= n && n2 % 12 == 7 {
                is_prime[n2] = !is_prime[n2];
            }

            // 条件3: n = 3x^2 - y^2
            // x > y かつ n % 12 == 11 の場合、判定を反転
            if x > y {
                let n3 = 3 * xx - yy;
                if n3 <= n && n3 % 12 == 11 {
                    is_prime[n3] = !is_prime[n3];
                }
            }

            y += 1;
        }
        x += 1;
    }

    // --- ステップ2: 素数の二乗の倍数を削除 ---
    let mut r = 5;
    while r * r <= n {
        if is_prime[r] {
            let mut k = r * r;
            while k <= n {
                is_prime[k] = false;
                k += r * r;
            }
        }
        r += 1;
    }

    is_prime
}

fn prime_list(n: usize) -> Vec<usize> {
    let is_prime = prime_sieve(n);
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}
