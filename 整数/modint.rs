use ac_library::ModInt998244353 as Mint;

/// 組み合わせ計算用前計算
pub struct Comb {
    fact: Vec<Mint>,
    inv_fact: Vec<Mint>,
    max_n: usize,
}

impl Comb {
    /// n以下の階乗と階乗の逆元を前計算
    pub fn new(n: usize) -> Self {
        let mut fact = vec![Mint::raw(1); n + 1];
        // 階乗を計算
        for i in 1..=n {
            fact[i] = fact[i - 1] * Mint::new(i as i64);
        }
        let mut inv_fact = vec![Mint::raw(1); n + 1];
        // 最大の階乗の逆元を計算
        inv_fact[n] = fact[n].inv();
        // 階乗の逆元を逆順に計算
        for i in (1..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * Mint::new(i as i64);
        }
        Self {
            fact,
            inv_fact,
            max_n: n,
        }
    }

    /// n!を計算
    pub fn factorial(&self, n: usize) -> Mint {
        assert!(n <= self.max_n);
        self.fact[n]
    }

    /// nPrを計算
    pub fn permutation(&self, n: usize, r: usize) -> Mint {
        if n < r {
            return Mint::raw(0);
        }
        assert!(n <= self.max_n);
        self.fact[n] * self.inv_fact[n - r]
    }

    /// nCrを計算
    pub fn combination(&self, n: usize, r: usize) -> Mint {
        if n < r {
            return Mint::raw(0);
        }
        assert!(n <= self.max_n);
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }

    /// nHr（重複組み合わせ）を計算
    pub fn homogeneous(&self, n: usize, r: usize) -> Mint {
        if n == 0 && r > 0 {
            Mint::raw(0)
        } else {
            self.combination(n + r - 1, r)
        }
    }

    /// カタラン数 C_n = C(2n, n) / (n+1) を計算
    pub fn catalan(&self, n: usize) -> Mint {
        if n == 0 {
            Mint::raw(1)
        } else {
            self.combination(2 * n, n) / Mint::new((n + 1) as i64)
        }
    }

    /// 階乗の逆元を取得
    pub fn inv_factorial(&self, n: usize) -> Mint {
        assert!(n <= self.max_n);
        self.inv_fact[n]
    }
}

// 簡易版関数群（後方互換）
pub fn factorial(n: usize) -> Mint {
    let mut acc = Mint::raw(1);
    for i in 1..=n {
        acc *= Mint::new(i as i64);
    }
    acc
}

pub fn permutation(n: usize, r: usize) -> Mint {
    if n < r {
        return Mint::raw(0);
    }
    let mut acc = Mint::raw(1);
    for i in n - r + 1..=n {
        acc *= Mint::new(i as i64);
    }
    acc
}

pub fn combination(n: usize, r: usize) -> Mint {
    if n < r {
        return Mint::raw(0);
    }
    permutation(n, r) / factorial(r)
}

pub fn homogeneous(n: usize, r: usize) -> Mint {
    combination(n + r - 1, r)
}
