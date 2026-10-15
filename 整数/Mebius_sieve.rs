// mobius_sieve(n): μ(1), μ(2), ..., μ(n) を返す (O(n))
pub fn mobius_sieve(n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; n + 1];
    if n == 0 {
        return mu;
    }
    mu[1] = 1;

    let mut lp = vec![0u32; n + 1]; // least prime factor
    let mut primes: Vec<u32> = Vec::new();

    for i in 2..=n {
        if lp[i] == 0 {
            lp[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1; // i は素数
        }
        for &p in &primes {
            let x = i * p as usize;
            if x > n {
                break;
            }
            lp[x] = p;
            if (i as u32) % p == 0 {
                mu[x] = 0; // p^2 | x
                break;
            } else {
                mu[x] = -mu[i];
            }
        }
    }
    mu
}

pub fn mobius_of(mut n: u64) -> i8 {
    if n == 1 {
        return 1;
    }
    let mut mu: i8 = 1;

    // 素因数 2 を処理
    if n % 2 == 0 {
        n /= 2;
        if n % 2 == 0 {
            return 0;
        } // 2^2 | n
        mu = -mu;
        while n % 2 == 0 {
            n /= 2;
        } // 念のため全て落とす
    }

    // 奇数素因数を処理
    let mut p: u64 = 3;
    while p * p <= n {
        if n % p == 0 {
            n /= p;
            if n % p == 0 {
                return 0;
            } // p^2 | n
            mu = -mu;
            while n % p == 0 {
                n /= p;
            }
        }
        p += 2;
    }

    // 最後に n>1 なら素因数が1個残っている
    if n > 1 {
        mu = -mu;
    }
    mu
}
