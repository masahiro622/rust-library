fn modpow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }
    result
}

fn legendre_symbol(a: u64, p: u64) -> u64 {
    modpow(a, (p - 1) / 2, p)
}

fn sqrt_mod(n: u64, p: u64) -> Option<u64> {
    if n % p == 0 {
        return Some(0);
    }
    if legendre_symbol(n, p) != 1 {
        return None; // 解が存在しない
    }
    if p == 2 {
        return Some(n);
    }
    if p % 4 == 3 {
        return Some(modpow(n, (p + 1) / 4, p));
    }

    // p-1 = Q * 2^S を見つける
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // 非二次剰余 z を見つける
    let mut z = 2;
    while legendre_symbol(z, p) != p - 1 {
        z += 1;
    }

    let mut c = modpow(z, q, p);
    let mut r = modpow(n, (q + 1) / 2, p);
    let mut t = modpow(n, q, p);
    let mut m = s;

    while t != 1 {
        let mut i = 1;
        let mut t2i = t * t % p;
        while t2i != 1 {
            t2i = t2i * t2i % p;
            i += 1;
            if i == m {
                return None; // 解が存在しない
            }
        }

        let b = modpow(c, 1 << (m - i - 1), p);
        r = r * b % p;
        t = t * b % p * b % p;
        c = b * b % p;
        m = i;
    }

    Some(r)
}
