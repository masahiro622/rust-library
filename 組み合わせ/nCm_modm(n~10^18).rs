// nCk mod m (m is not necessarily prime)
// 前処理 O(m) クエリ O(p log n) p: mの素因数　くらい？
// 1 <= n <= 10^18, 1 <= k <= 10^18, 1 <= m <= 10^6
// 2*10^5 回程度のクエリまでなら間に合う

pub fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in (2..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

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
    let mut m = m as i64;
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

fn is_composite_fast(n: usize, a: usize, d: usize, s: u32) -> bool {
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

pub fn is_prime_fast(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // n - 1 = d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    for &a in bases {
        if a >= n {
            break;
        }
        if is_composite_fast(n, a, d, s) {
            return false;
        }
    }

    true
}

use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

fn pollard_rho(n: usize) -> usize {
    if n % 2 == 0 {
        return 2;
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = SimpleRng::new(seed as usize);

    loop {
        let c = rng.gen_range(1, n);
        let mut x = rng.gen_range(2, n);
        let mut y = x;
        let mut d = 1usize;
        let f = |x: usize| (mod_mul(x, x, n) + c) % n;

        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd((x as i64 - y as i64).abs() as usize, n);
        }

        if d != n {
            return d;
        }
    }
}
/// ユークリッドの互除法
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// 線形合同法による簡易乱数生成器
struct SimpleRng {
    state: usize,
}

impl SimpleRng {
    /// 新しい乱数生成器を初期化
    fn new(seed: usize) -> Self {
        SimpleRng { state: seed }
    }

    /// 次の乱数を生成
    fn next(&mut self) -> usize {
        // 線形合同法のパラメータ
        const A: usize = 6364136223846793005;
        const C: usize = 1;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    /// 指定した範囲の乱数を生成
    fn gen_range(&mut self, start: usize, end: usize) -> usize {
        assert!(start < end, "start must be less than end");
        start + (self.next() % (end - start))
    }
}
use std::collections::HashMap;

/// 素因数分解（Map or Vecなどで自由に拡張可能）
fn factorize(mut n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    if n <= 1 {
        return res;
    }
    let mut stack = vec![n];

    while let Some(n) = stack.pop() {
        if n == 1 {
            continue;
        }
        if is_prime_fast(n) {
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

fn factorize_map(n: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    let fastors = factorize(n);
    for &f in &fastors {
        *res.entry(f).or_insert(0) += 1;
    }
    res
}

// 修正版 - pを含まない階乗値も事前計算
fn prepare_factorials_without_p(p: usize, e: usize) -> (Vec<usize>) {
    let pe = p.pow(e as u32);
    let mut fact_without_p = vec![1; pe];

    // 階乗の計算
    for i in 1..pe {
        if i % p == 0 {
            fact_without_p[i] = fact_without_p[i - 1];
        } else {
            fact_without_p[i] = (fact_without_p[i - 1] * i) % pe;
        }
    }

    fact_without_p
}

// n! の中に含まれる p の冪数を求める
fn count_p(mut n: usize, p: usize) -> usize {
    let mut res = 0;
    while n > 0 {
        n /= p;
        res += n;
    }
    res
}

fn factorial_without_p_fast(n: usize, p: usize, pe: usize, fact_without_p: &[usize]) -> usize {
    if n == 0 {
        return 1;
    }

    let cycle_length = pe;
    let full_cycles = n / cycle_length;
    let remainder = n % cycle_length;

    // 1つのサイクル内でのpを含まない階乗
    let cycle_factorial = fact_without_p[cycle_length - 1];

    // full_cycles回のサイクルの貢献
    let mut result = mod_pow(cycle_factorial, full_cycles, pe);

    // 残りの部分の貢献
    if remainder > 0 {
        result = (result * fact_without_p[remainder]) % pe;
    }

    // 再帰的な部分 n/p!
    if n >= p {
        let recursive = factorial_without_p_fast(n / p, p, pe, fact_without_p);
        result = (result * recursive) % pe;
    }

    result
}

// 拡張Lucas対応 comb_mod_pe（修正版）
// 修正された comb_mod_pe
fn comb_mod_pe(n: usize, k: usize, p: usize, e: usize, fact_without_p: &[usize]) -> usize {
    if k > n {
        return 0;
    }

    let pe = p.pow(e as u32);
    let t = count_p(n, p) - count_p(k, p) - count_p(n - k, p);
    if t >= e {
        return 0;
    }

    let a = factorial_without_p_fast(n, p, pe, fact_without_p);
    let b = factorial_without_p_fast(k, p, pe, fact_without_p);
    let c = factorial_without_p_fast(n - k, p, pe, fact_without_p);

    let denom = (b * c) % pe;
    let num = a;
    let denom_inv = mod_inv(denom, pe);
    let res = (num * denom_inv) % pe;
    (res * mod_pow(p, t, pe)) % pe
}

// Garner's algorithm for CRT reconstruction
// 標準的なCRT実装
fn chinese_remainder_theorem(residues: &[usize], mods: &[usize]) -> usize {
    let n = residues.len();
    if n == 0 {
        return 0;
    }

    let mut result = residues[0];
    let mut lcm = mods[0];

    for i in 1..n {
        // result ≡ residues[i] (mod mods[i]) を満たすように更新
        let a = residues[i];
        let m = mods[i];

        // result + k * lcm ≡ a (mod m) を解く
        // k * lcm ≡ (a - result) (mod m)
        let diff = (a + m - result % m) % m;
        let g = gcd(lcm, m);

        if diff % g != 0 {
            // 解が存在しない
            return 0;
        }

        let lcm_div_g = lcm / g;
        let m_div_g = m / g;
        let diff_div_g = diff / g;

        let k = (diff_div_g * mod_inv(lcm_div_g, m_div_g)) % m_div_g;
        result += k * lcm;
        lcm = lcm * m_div_g; // lcm(lcm, m)
    }

    result
}

// m は素因数分解を受け取る
fn binom_mod_m(
    n: usize,
    k: usize,
    m: &HashMap<usize, usize>,
    precomputed: &HashMap<(usize, usize), Vec<usize>>,
) -> usize {
    // k > n の場合は 0
    if k > n {
        return 0;
    }

    let mut residues = vec![];
    let mut moduli = vec![];

    for (&p, &e) in m {
        let pe = p.pow(e as u32);
        let fact_without_p = &precomputed[&(p, e)];
        let r = comb_mod_pe(n, k, p, e, fact_without_p);

        residues.push(r);
        moduli.push(pe);
    }
    let result = chinese_remainder_theorem(&residues, &moduli);
    result
}

use std::{io, result, vec};

fn read_buffer() -> Vec<usize> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse."))
        .collect()
}
