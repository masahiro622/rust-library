use ac_library::{FenwickTree, ModInt998244353};
use std::ops::{Add, AddAssign, Bound, Mul, Neg, RangeBounds, Sub, SubAssign};

/// 汎用化のためのヘルパートレイト
/// usize (インデックス) を型 T に変換する機能を保証します
pub trait Scalar: Sized {
    fn from_usize(n: usize) -> Self;
}

// i64 用の実装
impl Scalar for i64 {
    fn from_usize(n: usize) -> Self {
        n as i64
    }
}

// ModInt 用の実装 (ac-library-rsのModIntはFrom<usize>を持っている)
impl Scalar for ModInt998244353 {
    fn from_usize(n: usize) -> Self {
        Self::from(n)
    }
}

// 必要に応じて他のModIntもここに追加
// impl Scalar for ModInt1000000007 { ... }

/// 区間加算・区間和 (Range Add Range Sum)
/// T: 保持する値の型 (i64 や ModInt)
pub struct ExFenwickTree<T> {
    n: usize,
    p: FenwickTree<T>,
    q: FenwickTree<T>,
}

impl<T> ExFenwickTree<T>
where
    T: Clone
        + Default
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AddAssign
        + SubAssign
        + Scalar, // 自作トレイト
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            p: FenwickTree::new(n + 1, T::default()),
            q: FenwickTree::new(n + 1, T::default()),
        }
    }

    /// 区間 [l, r) に w を加算
    pub fn add(&mut self, l: usize, r: usize, w: T) {
        if l >= r {
            return;
        }
        let l_t = T::from_usize(l);
        let r_t = T::from_usize(r);

        // p.add(l, -w * l);
        self.p.add(l, -w * l_t);
        // p.add(r, w * r);
        self.p.add(r, w * r_t);
        // q.add(l, w);
        self.q.add(l, w);
        // q.add(r, -w);
        self.q.add(r, -w);
    }

    /// [0, r) の和を取得する内部ヘルパー
    fn prefix_sum(&self, r: usize) -> T {
        // 公式: sum(p, r) + sum(q, r) * r
        let r_t = T::from_usize(r);
        self.p.sum(0..r) + self.q.sum(0..r) * r_t
    }

    /// 任意の区間の和を取得
    /// 範囲は 0..n, ..r, l..=r など RangeBounds で指定可能
    pub fn sum<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.n,
        };

        if l >= r {
            return T::default();
        }

        self.prefix_sum(r) - self.prefix_sum(l)
    }
}

fn main() {
    // --- i64 での使用例 ---
    let n = 10;
    let mut tree_i64 = ExFenwickTree::<i64>::new(n);
    tree_i64.add(0, 5, 10); // [0, 5) += 10
    tree_i64.add(3, 8, 5); // [3, 8) += 5
    println!("i64 sum [0..5): {}", tree_i64.sum(0..5)); // 60

    // --- ModInt での使用例 ---
    use ac_library::ModInt998244353 as Mint;
    let mut tree_mint = ExFenwickTree::<Mint>::new(n);
    tree_mint.add(0, 5, 10.into());
    tree_mint.add(3, 8, 5.into());
    println!("Mint sum [0..5): {}", tree_mint.sum(0..5)); // 60
}
