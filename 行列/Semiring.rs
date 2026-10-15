use std::ops::{Add, Mul};

/// 半環 (Semiring) トレイト
/// 足し算っぽい演算 (add) と 掛け算っぽい演算 (mul) を定義する
pub trait SemiRing: Clone {
    type T: Clone + Copy; // 競プロでは主にプリミティブ型(i64, usize)を使うためCopyを要求
    fn zero() -> Self::T; // 足し算の単位元 (例: 0, 無限大)
    fn one() -> Self::T; // 掛け算の単位元 (例: 1, 0)
    fn add(a: Self::T, b: Self::T) -> Self::T;
    fn mul(a: Self::T, b: Self::T) -> Self::T;
}

#[derive(Clone, Debug)]
pub struct Matrix<S: SemiRing> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<S::T>>,
}

impl<S: SemiRing> Matrix<S> {
    /// 指定されたサイズで初期化（全要素 zero）
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![vec![S::zero(); cols]; rows],
        }
    }

    /// 単位行列を生成
    pub fn identity(size: usize) -> Self {
        let mut mat = Self::new(size, size);
        for i in 0..size {
            mat.data[i][i] = S::one();
        }
        mat
    }

    /// 行列の乗算 (O(N^3))
    pub fn mat_mul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "行列の次元が一致しません");
        let mut res = Self::new(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                // 疎行列などで0スキップしたい場合はここで分岐可能
                // if self.data[i][k] == S::zero() { continue; }
                for j in 0..other.cols {
                    let val = S::mul(self.data[i][k], other.data[k][j]);
                    res.data[i][j] = S::add(res.data[i][j], val);
                }
            }
        }
        res
    }

    /// 行列累乗 (繰り返し二乗法) (O(N^3 log K))
    pub fn pow(&self, mut k: u64) -> Self {
        assert_eq!(self.rows, self.cols, "正方行列ではありません");
        let mut res = Self::identity(self.rows);
        let mut base = (*self).clone();
        while k > 0 {
            if k & 1 == 1 {
                res = res.mat_mul(&base);
            }
            base = base.mat_mul(&base);
            k >>= 1;
        }
        res
    }
}

// ----------------------------------------------------------------------
// 使用例
// ----------------------------------------------------------------------

// 例1: 通常の行列累乗 (フィボナッチ数などで使用) mod 998244353

#[derive(Clone)]
struct ModularRing;
const MOD: i64 = 998_244_353;

impl SemiRing for ModularRing {
    type T = i64;
    fn zero() -> i64 {
        0
    }
    fn one() -> i64 {
        1
    }
    fn add(a: i64, b: i64) -> i64 {
        (a + b) % MOD
    }
    fn mul(a: i64, b: i64) -> i64 {
        (a * b) % MOD
    }
}

// 例2: トロピカル半環 (Min-Plus代数)
// グラフのKステップ後の最短経路問題などで使用
// 足し算 -> min, 掛け算 -> + に置き換わる

#[derive(Clone)]
struct TropicalRing;
const INF: i64 = i64::MAX;

impl SemiRing for TropicalRing {
    type T = i64;
    fn zero() -> i64 {
        INF
    } // minの単位元は無限大
    fn one() -> i64 {
        0
    } // +の単位元は0
    fn add(a: i64, b: i64) -> i64 {
        a.min(b)
    }
    fn mul(a: i64, b: i64) -> i64 {
        a.saturating_add(b)
    }
}

#[derive(Clone)]
struct XorAndRing;

impl SemiRing for XorAndRing {
    type T = i64;
    fn zero() -> i64 {
        0
    } // XORの単位元は0
    fn one() -> i64 {
        1
    } // ANDの単位元は1
    fn add(a: i64, b: i64) -> i64 {
        a ^ b
    }
    fn mul(a: i64, b: i64) -> i64 {
        a & b
    }
}

#[derive(Clone)]
struct OrAndRing;

impl SemiRing for OrAndRing {
    type T = i64;
    fn zero() -> i64 {
        0
    } // ORの単位元は0
    fn one() -> i64 {
        1
    } // ANDの単位元は1
    fn add(a: i64, b: i64) -> i64 {
        a | b
    }
    fn mul(a: i64, b: i64) -> i64 {
        a & b
    }
}
