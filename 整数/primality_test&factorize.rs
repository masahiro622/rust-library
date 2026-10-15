use num_integer::gcd;

// n < 2^64 ≒ 10^18のときis_prime_fast(O(N^(1/4)))が使える
// factorize : 素因数分解を行う(O(N^(1/4)))

fn mod_mul(a: usize, b: usize, m: usize) -> usize {
    ((a as u128 * b as u128) % m as u128) as usize
}

fn mod_pow(mut base: usize, mut exp: usize, modu: usize) -> usize {
    let mut result = 1usize;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp /= 2;
    }
    result
}

fn mod_inv(a: usize, m: usize) -> usize {
    let mut a = a as i64;
    let m = m as i64;
    let mut b = m;
    let mut x = 1;
    let mut y = 0;
    while a != 0 {
        let q = b / a;
        b -= q * a;
        std::mem::swap(&mut a, &mut b);
        y -= q * x;
        std::mem::swap(&mut x, &mut y);
    }
    assert!(b == 1);
    ((y + m) % m) as usize
}

fn is_composite(n: usize, a: usize, d: usize, s: u32) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    for _ in 0..s - 1 {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return false;
        }
    }
    true
}

pub fn is_prime(n: usize) -> bool {
    if n <= 2 {
        return n == 2;
    }
    if n % 2 == 0 {
        return false;
    }

    // n - 1 = d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while (d & 1) == 0 {
        d >>= 1;
        s += 1;
    }

    const BASES: [usize; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

    for &a in &BASES {
        let a = a % n;
        if a == 0 {
            return true;
        }
        if is_composite(n, a, d, s) {
            return false;
        }
    }
    true
}

use std::time::{SystemTime, UNIX_EPOCH};

fn pollard_rho(n: usize) -> usize {
    if (n & 1) == 0 {
        return 2;
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let mut rng = XorShift64::new(seed ^ 0x9E3779B97F4A7C15);
    loop {
        let c = rng.gen_range(1, n);
        let mut y = rng.gen_range(1, n);
        let mut r: usize = 1;
        let m: usize = 128;

        let mut g: usize = 1;
        let mut q: usize = 1;
        let mut x: usize = 0;
        let mut ys: usize = 0;

        #[inline]
        fn f(x: usize, c: usize, n: usize) -> usize {
            (mod_mul(x, x, n) + c) % n
        }

        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y, c, n);
            }
            let mut k: usize = 0;
            while k < r && g == 1 {
                ys = y;
                let lim = (r - k).min(m);
                for _ in 0..lim {
                    y = f(y, c, n);
                    let diff = x.abs_diff(y);
                    q = mod_mul(q, diff, n);
                }
                g = gcd(q, n);
                k += lim;
            }
            r <<= 1;
        }

        if g == n {
            // fallback
            loop {
                ys = f(ys, c, n);
                let diff = x.abs_diff(ys);
                g = gcd(diff, n);
                if g > 1 {
                    break;
                }
            }
        }

        if g != n {
            return g;
        }
    }
}

// 安全な乗算（limit以下に収まる場合のみ返す）
fn safe_mul(a: usize, b: usize, limit: usize) -> Option<usize> {
    a.checked_mul(b)
        .and_then(|x| if x <= limit { Some(x) } else { None })
}

struct XorShift64 {
    x: usize,
}
impl XorShift64 {
    #[inline]
    fn new(seed: usize) -> Self {
        let mut s = seed;
        if s == 0 {
            s = 0xdead_beef_cafe_babe;
        }
        Self { x: s }
    }
    #[inline]
    fn next_u64(&mut self) -> usize {
        let mut x = self.x;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.x = x;
        x
    }
    #[inline]
    fn gen_range(&mut self, lo: usize, hi: usize) -> usize {
        debug_assert!(lo < hi);
        lo + (self.next_u64() % (hi - lo))
    }
}

/// 素因数分解（Map or Vecなどで自由に拡張可能）
fn factorize(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    if n <= 1 {
        return res;
    }
    let mut stack = vec![n];

    while let Some(n) = stack.pop() {
        if n == 1 {
            continue;
        }
        if is_prime(n) {
            res.push(n);
        } else {
            let d = pollard_rho(n);
            stack.push(d);
            stack.push(n / d);
        }
    }

    res.sort_unstable();
    res
}

fn factorize_map(n: usize) -> rustc_hash::FxHashMap<usize, usize> {
    let res: rustc_hash::FxHashMap<_, _> = factorize(n)
        .iter()
        .dedup_with_count()
        .map(|(c, d)| (*d, c))
        .collect();
    res
}

fn divisors(n: usize) -> Vec<usize> {
    let factors = factorize_map(n);
    let mut divisors = vec![1];
    for (p, exp) in factors {
        let mut new_divs = Vec::new();
        for &d in &divisors {
            let mut val = 1;
            for _ in 0..=exp {
                // d * val がオーバーフローまたは n を超える場合はスキップ
                if let Some(prod) = safe_mul(d, val, n) {
                    new_divs.push(prod);
                }
                // val *= p を安全に更新（オーバーフロー時は以降不可能なので打ち切り）
                if let Some(next) = safe_mul(val, p, n) {
                    val = next;
                } else {
                    break;
                }
            }
        }
        divisors = new_divs;
    }
    // 結果を整形
    divisors.sort_unstable();
    divisors.dedup();
    divisors
}
