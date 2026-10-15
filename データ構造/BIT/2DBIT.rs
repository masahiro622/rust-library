use std::ops::{Add, AddAssign, Bound, Mul, Neg, RangeBounds, Sub, SubAssign};

/// 汎用化のためのヘルパートレイト
pub trait Scalar: Sized {
    fn from_usize(n: usize) -> Self;
}

impl Scalar for i64 {
    fn from_usize(n: usize) -> Self {
        n as i64
    }
}

// ac-library-rsのModIntがあれば有効化
use ac_library::ModInt998244353;
impl Scalar for ModInt998244353 {
    fn from_usize(n: usize) -> Self {
        Self::from(n)
    }
}

/// 2次元BIT (Point Add, Rectangle Sum)
#[derive(Clone, Debug)]
pub struct FenwickTree2D<T> {
    h: usize,
    w: usize,
    data: Vec<T>, // (h+1)*(w+1) のサイズで平坦化
}

impl<T> FenwickTree2D<T>
where
    T: Clone + Default + Copy + AddAssign + Add<Output = T> + Sub<Output = T> + Scalar,
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![T::default(); (h + 1) * (w + 1)],
        }
    }

    /// (i, j) に x を加算 (0-indexed)
    pub fn add(&mut self, i: usize, j: usize, x: T) {
        let mut r = i + 1;
        while r <= self.h {
            let mut c = j + 1;
            while c <= self.w {
                // data[r][c] += x
                let idx = r * (self.w + 1) + c;
                self.data[idx] += x;
                c += (c as isize & -(c as isize)) as usize;
            }
            r += (r as isize & -(r as isize)) as usize;
        }
    }

    /// [0, i) x [0, j) の矩形和を取得 (0-indexed)
    fn prefix_sum(&self, i: usize, j: usize) -> T {
        let mut res = T::default();
        let mut r = i;
        while r > 0 {
            let mut c = j;
            while c > 0 {
                let idx = r * (self.w + 1) + c;
                res += self.data[idx];
                c -= (c as isize & -(c as isize)) as usize;
            }
            r -= (r as isize & -(r as isize)) as usize;
        }
        res
    }

    /// 任意の矩形領域の和を取得
    /// row_range: 行の範囲 (例: 1..3)
    /// col_range: 列の範囲 (例: 2..5)
    pub fn sum<R1, R2>(&self, row_range: R1, col_range: R2) -> T
    where
        R1: RangeBounds<usize>,
        R2: RangeBounds<usize>,
    {
        let r1 = match row_range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let r2 = match row_range.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.h,
        };

        let c1 = match col_range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let c2 = match col_range.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.w,
        };

        if r1 >= r2 || c1 >= c2 {
            return T::default();
        }

        // 包除原理: S(r2, c2) - S(r1, c2) - S(r2, c1) + S(r1, c1)
        self.prefix_sum(r2, c2) - self.prefix_sum(r1, c2) - self.prefix_sum(r2, c1)
            + self.prefix_sum(r1, c1)
    }
}

fn main() {
    let h = 4;
    let w = 4;
    let mut ft = FenwickTree2D::<i64>::new(h, w);

    // (1, 1) に 10 を加算
    ft.add(1, 1, 10);
    // (2, 2) に 5 を加算
    ft.add(2, 2, 5);

    // 矩形 [0, 3) x [0, 3) の和 -> 10 + 5 = 15
    println!("Sum [0..3)x[0..3): {}", ft.sum(0..3, 0..3));

    // 矩形 [1, 2) x [1, 2) の和 -> (1,1)のみ含む -> 10
    println!("Sum [1..2)x[1..2): {}", ft.sum(1..2, 1..2));

    // 矩形 [2, 3) x [2, 3) の和 -> (2,2)のみ含む -> 5
    println!("Sum [2..3)x[2..3): {}", ft.sum(2..3, 2..3));
}
