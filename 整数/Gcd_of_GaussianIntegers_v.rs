// ガウス整数の最大公約数を求める関数
// さらに、与えられた整数 n を a^2 + b^2 の形で表す全ての非負整数の組 (a, b) を列挙する関数も実装
// requires: factorize_map(n) -> HashSet<(usize, usize)>
use std::collections::HashSet;

// 複素数（ガウス整数）を扱うための構造体
#[derive(Clone, Copy, Debug)]
struct Gauss {
    x: i128,
    y: i128,
}

impl Gauss {
    fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }

    // 複素数の掛け算: (a+bi)(c+di) = (ac-bd) + (ad+bc)i
    fn mul(self, rhs: Gauss) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }

    // ノルム（絶対値の2乗）: x^2 + y^2
    fn norm_sq(self) -> i128 {
        self.x * self.x + self.y * self.y
    }

    fn conj(self) -> Gauss {
        Gauss {
            x: self.x,
            y: -self.y,
        }
    }

    fn div_rem(a: Gauss, b: Gauss) -> (Gauss, Gauss) {
        let den = b.norm_sq();
        let conj_b = b.conj();
        let num = a.mul(conj_b);

        // 四捨五入（最近整数）
        let q_x = ((num.x as f64) / (den as f64)).round() as i128;
        let q_y = ((num.y as f64) / (den as f64)).round() as i128;
        let q = Gauss { x: q_x, y: q_y };

        let r = Gauss {
            x: a.x - (q.x * b.x - q.y * b.y),
            y: a.y - (q.x * b.y + q.y * b.x),
        };

        (q, r)
    }

    fn gcd(mut a: Gauss, mut b: Gauss) -> Gauss {
        while b.x != 0 || b.y != 0 {
            let (_, r) = Gauss::div_rem(a, b);
            a = b;
            b = r;
        }
        // 一意性のため、正規化（ノルム最小に）
        let xps = [
            a,
            Gauss { x: -a.x, y: -a.y },
            Gauss { x: -a.y, y: a.x }, // i 倍
            Gauss { x: a.y, y: -a.x }, // -i 倍
        ];
        *xps.iter().min_by_key(|g| (g.x.abs(), g.y.abs())).unwrap()
    }
}

fn two_square_sum(n: usize) -> Vec<(usize, usize)> {
    // 0の場合は特例 (0^2 + 0^2 = 0)
    if n == 0 {
        return vec![(0, 0)];
    }

    // 1. 素因数分解
    let factors = factorize_map(n);

    // 2. 解の存在判定 & 確定成分の計算
    // 4k+3型の素数の指数が奇数なら解なし
    let mut base_gaussian = Gauss::new(1, 0);
    let mut split_primes = Vec::new();

    for (p, exp) in factors {
        if p == 2 {
            // 2 = (1+i)(1-i) = -i(1+i)^2
            // 2^k の成分は (1+i)^k を掛けるだけでよい（分岐しない）
            let g = Gauss::new(1, 1); // 1+i
                                      // (1+i)^exp を計算
            for _ in 0..exp {
                base_gaussian = base_gaussian.mul(g);
            }
        } else if p % 4 == 3 {
            // 4k+3型
            if exp % 2 != 0 {
                // 指数が奇数なら和で表せない
                return Vec::new();
            }
            // 指数が偶数なら p^(exp/2) を実数成分として掛ける
            let scale = p.pow((exp / 2) as u32) as i128;
            base_gaussian = base_gaussian.mul(Gauss::new(scale, 0));
        } else {
            // 4k+1型: ガウス素数に分解して分岐探索する
            split_primes.push((p, exp));
        }
    }

    // 3. 4k+1型素数の組み合わせをDFSで全探索
    let mut solutions = HashSet::new();
    dfs(0, base_gaussian, &split_primes, &mut solutions);

    // 4. 出力 (ソートして一意にする)
    let mut ans_vec: Vec<(usize, usize)> = solutions.into_iter().collect();
    ans_vec.sort_unstable();
    ans_vec
}

// 深さ優先探索でガウス素数の組み合わせを列挙
fn dfs(
    idx: usize,
    current: Gauss,
    split_primes: &[(usize, usize)],
    results: &mut HashSet<(usize, usize)>,
) {
    if idx == split_primes.len() {
        // 探索終了: 実部と虚部の絶対値をとって解とする
        let a = current.x.abs() as usize;
        let b = current.y.abs() as usize;
        results.insert((a, b));
        results.insert((b, a)); // 順序入れ替えも追加
        return;
    }

    let (p, exponent) = split_primes[idx];

    // p = u^2 + v^2 を満たす (u, v) を見つける (Cornacchia's algorithm)
    let (u, v) = cornacchia(p);

    // ガウス素数 pi = u + vi
    let pi = Gauss::new(u as i128, v as i128);
    let pi_bar = Gauss::new(u as i128, -(v as i128));

    // p^e の約数は、pi^j * pi_bar^(e-j) (j=0..e) の形になる
    // それぞれのケースについて再帰

    // 事前計算: pi と pi_bar の累乗リスト
    let mut pi_pows = vec![Gauss::new(1, 0); exponent + 1];
    let mut pi_bar_pows = vec![Gauss::new(1, 0); exponent + 1];

    for i in 0..exponent {
        pi_pows[i + 1] = pi_pows[i].mul(pi);
        pi_bar_pows[i + 1] = pi_bar_pows[i].mul(pi_bar);
    }

    for j in 0..=exponent {
        let k = exponent - j;
        let factor = pi_pows[j].mul(pi_bar_pows[k]);
        let next_val = current.mul(factor);
        dfs(idx + 1, next_val, split_primes, results);
    }
}

// Cornacchia's algorithm: p = x^2 + y^2 を満たす x, y を見つける
// p は 4k+1 型の素数である必要がある
fn cornacchia(p: usize) -> (usize, usize) {
    // 1. r^2 ≡ -1 (mod p) となる r を見つける
    let r = find_sqrt_neg_one(p);

    // 2. ユークリッドの互除法の変種
    // r0 = p, r1 = r として、r_k < sqrt(p) になるまで剰余を取る
    let mut r0 = p;
    let mut r1 = r;

    // target = sqrt(p)
    let limit = (p as f64).sqrt() as usize;

    while r1 > limit {
        let rem = r0 % r1;
        r0 = r1;
        r1 = rem;
    }

    let x = r1;
    let y = ((p - x * x) as f64).sqrt().round() as usize;
    (x, y)
}

// r^2 ≡ -1 (mod p) となる r を探す (Tonelli-Shanksの簡易版)
// p ≡ 1 (mod 4) の時のみ有効
fn find_sqrt_neg_one(p: usize) -> usize {
    // ランダムに z を選び、z^((p-1)/4) を計算する
    // これが r になる確率は 50% 程度なので数回で求まる

    // 決定論的に探索しても十分速い
    let mut z = 2;
    loop {
        // オイラーの基準: z^((p-1)/2) ≡ -1 (mod p) なら z は平方非剰余
        // このとき r = z^((p-1)/4) とすると r^2 ≡ -1
        let exp = (p - 1) / 4;
        let r = mod_pow(z, exp, p);

        // r^2 ≡ -1 (mod p) か確認
        if mod_pow(r, 2, p) == p - 1 {
            return r;
        }
        z += 1;
    }
}
