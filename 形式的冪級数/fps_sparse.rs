const MOD: u32 = 998_244_353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ModInt(u32);

impl ModInt {
    fn new(val: u32) -> Self {
        ModInt(val % MOD)
    }

    fn zero() -> Self {
        ModInt(0)
    }

    fn one() -> Self {
        ModInt(1)
    }

    fn inv(self) -> Self {
        self.pow(MOD as u64 - 2)
    }

    fn pow(self, mut e: u64) -> Self {
        let mut base = self.0 as u64;
        let mut res = 1u64;
        while e > 0 {
            if e % 2 == 1 {
                res = res * base % MOD as u64;
            }
            base = base * base % MOD as u64;
            e /= 2;
        }
        ModInt(res as u32)
    }
}

impl ModInt {
    #[inline]
    fn from_u64(v: u64) -> Self {
        ModInt(((v % MOD as u64) as u32))
    }
    #[inline]
    fn from_usize(v: usize) -> Self {
        ModInt::new(v as u32)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::ops::{Add, Mul, Neg, Sub};

impl Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: Self) -> Self::Output {
        ModInt::new((self.0 + rhs.0) % MOD)
    }
}

impl Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> Self::Output {
        ModInt::new((MOD + self.0 - rhs.0) % MOD)
    }
}

impl Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt::new((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32)
    }
}

impl Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> Self::Output {
        ModInt::new((MOD - self.0) % MOD)
    }
}

/// 疎な形式的冪級数の逆数を先頭 `deg` 項まで求める
/// f は (次数, 係数) のベクタ。f[0].0 == 0 かつ f[0].1 != 0 が前提。
fn sparse_inv(f: &Vec<(usize, ModInt)>, deg: usize) -> Vec<ModInt> {
    assert!(!f.is_empty() && f[0].0 == 0 && f[0].1 != ModInt::zero());

    let mut g = vec![ModInt::zero(); deg];
    g[0] = f[0].1.inv(); // g0 = 1 / f0

    // 以降 g[n] = -1/f0 * sum_{i=1}^n f[i] * g[n - i]
    for n in 1..deg {
        let mut acc = ModInt::zero();
        for &(i, fi) in f.iter() {
            if i == 0 || i > n {
                continue;
            }
            acc = acc + fi * g[n - i];
        }
        g[n] = -acc * g[0];
    }

    g
}

/// 疎なFPS f(x) = [(deg, coef), ...] に対して、f'(x) を求める（疎）
fn sparse_derivative(f: &Vec<(usize, ModInt)>) -> Vec<(usize, ModInt)> {
    f.iter()
        .filter(|(d, _)| *d > 0)
        .map(|(d, c)| (*d - 1, ModInt::new(*d as u32) * *c))
        .collect()
}

/// 密な多項式を積分（g_0 = 0）して返す
fn integrate(f: &[ModInt]) -> Vec<ModInt> {
    let mut res = vec![ModInt::zero(); f.len() + 1];
    for i in 0..f.len() {
        res[i + 1] = f[i] * ModInt::new((i + 1) as u32).inv();
    }
    res
}

/// 疎な多項式 lhs（[deg, coef]）と密な rhs の畳み込み、先頭 `deg` 項まで
fn sparse_mul(lhs: &Vec<(usize, ModInt)>, rhs: &[ModInt], deg: usize) -> Vec<ModInt> {
    let mut res = vec![ModInt::zero(); deg];
    for &(i, a) in lhs.iter() {
        if a.0 == 0 {
            continue;
        }
        for j in 0..deg {
            if i + j >= deg {
                break;
            }
            res[i + j] = res[i + j] + a * rhs[j];
        }
    }
    res
}

/// log(f(x)) の先頭 `deg` 項を求める
fn sparse_log(f: &Vec<(usize, ModInt)>, deg: usize) -> Vec<ModInt> {
    assert!(!f.is_empty() && f[0].0 == 0 && f[0].1 != ModInt::zero());

    // 1. f'(x)
    let df = sparse_derivative(f);

    // 2. 1 / f(x)
    let inv_f = sparse_inv(f, deg);

    // 3. f'/f（先頭 deg 項まで）
    let df_div_f = sparse_mul(&df, &inv_f, deg);

    // 4. 積分
    integrate(&df_div_f)
}

fn sparse_exp(f: &Vec<(usize, ModInt)>, deg: usize) -> Vec<ModInt> {
    // f(x) = a_0 + a_1*x + a_2*x^2 + ... の exp を計算
    // exp(f) = g とすると、g' = f' * g という微分方程式を満たす

    // f(0) != 0 の場合はエラー（exp(f(0)) * exp(残り) の形で処理が必要）
    if !f.is_empty() && f[0].0 == 0 {
        panic!("sparse_exp: f(0) must be 0");
    }

    if deg == 0 {
        return vec![];
    }

    // 結果の多項式 g を格納
    let mut g = vec![ModInt::zero(); deg];
    g[0] = ModInt::one(); // g(0) = exp(f(0)) = exp(0) = 1

    if deg == 1 {
        return g;
    }

    // f の微分 f' を計算
    let f_prime = sparse_derivative(f);

    // 各次数について順次計算
    for i in 1..deg {
        let mut coeff = ModInt::zero();

        // g'[i-1] = (f' * g)[i-1] を計算
        // f'[j] * g[i-1-j] の和を取る
        for &(j, f_prime_j) in &f_prime {
            if j < i {
                coeff = coeff + f_prime_j * g[i - 1 - j];
            }
        }

        // g[i] = g'[i-1] / i = coeff / i
        g[i] = coeff * ModInt::new(i as u32).inv();
    }

    g
}

/// f0≠0 前提のコア（従来の実装を関数化）
fn sparse_pow_core(f: &Vec<(usize, ModInt)>, m: u64, deg: usize) -> Vec<ModInt> {
    assert!(!f.is_empty() && f[0].0 == 0 && f[0].1 != ModInt::zero());

    let mut g = vec![ModInt::zero(); deg];
    if deg == 0 {
        return g;
    }

    let f0 = f[0].1;
    g[0] = f0.pow(m);
    if deg == 1 {
        return g;
    }

    let inv_f0 = f0.inv();
    let m_mod = ModInt::from_u64(m);
    let pos: Vec<(usize, ModInt)> = f.iter().cloned().filter(|(d, _)| *d > 0).collect();

    for n in 0..deg - 1 {
        // (n+1) f0 g_{n+1} = Σ_{t>0, t≤n+1} a_t * (M*t - (n - t + 1)) * g_{n+1-t}
        let mut acc = ModInt::zero();
        for &(t, a_t) in &pos {
            if t > n + 1 {
                continue;
            }
            let coef = (m_mod * ModInt::new(t as u32) - ModInt::new((n + 1 - t) as u32)) * a_t;
            acc = acc + coef * g[n + 1 - t];
        }
        g[n + 1] = acc * ModInt::new((n as u32) + 1).inv() * inv_f0;
    }

    g
}

/// 疎なFPSの冪乗 g = f^M の先頭 `deg` 項を計算（f0=0 も可）
fn sparse_pow(f: &Vec<(usize, ModInt)>, m: u64, deg: usize) -> Vec<ModInt> {
    if deg == 0 {
        return vec![];
    }
    // m=0 は恒等的に 1
    if m == 0 {
        let mut g = vec![ModInt::zero(); deg];
        g[0] = ModInt::one();
        return g;
    }

    // 最初の非零項の次数 s と係数を探す（f が昇順でなくてもOK）
    if let Some((s, _a_s)) = f
        .iter()
        .filter(|&&(_d, c)| c != ModInt::zero())
        .min_by_key(|&&(d, _)| d)
    {
        if *s == 0 {
            // そのままコアで
            return sparse_pow_core(f, m, deg);
        }
        // f = x^s * h に分解
        let h: Vec<(usize, ModInt)> = f
            .iter()
            .filter(|&&(_d, c)| c != ModInt::zero())
            .map(|&(d, c)| (d - s, c))
            .collect();

        // シフト量 s*m が deg 以上なら全て 0（m>0 のため）
        let shift128 = (*s as u128) * (m as u128);
        if shift128 >= deg as u128 {
            return vec![ModInt::zero(); deg];
        }
        let shift = shift128 as usize;
        let mut g = vec![ModInt::zero(); deg];
        let tail = sparse_pow_core(&h, m, deg - shift);
        for (i, &v) in tail.iter().enumerate() {
            g[shift + i] = v;
        }
        g
    } else {
        // f ≡ 0 の場合
        // m>0 なら 0、m=0 は上で処理済み
        vec![ModInt::zero(); deg]
    }
}
