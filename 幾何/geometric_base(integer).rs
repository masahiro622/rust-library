use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

//------------------------------//
// 基本要素 (点, 線分, 直線, 円)
//------------------------------//

pub const INF: i64 = 1i64 << 60;

// Derive に PartialOrd, Ord を追加（辞書式順序による比較）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn conj(self) -> Self {
        Self::new(self.x, -self.y)
    }

    pub fn dot(self, r: Point) -> i64 {
        self.x * r.x + self.y * r.y
    }

    pub fn cross(self, r: Point) -> i64 {
        self.x * r.y - self.y * r.x
    }

    pub fn norm(self) -> i64 {
        self.dot(self)
    }

    // 2点間の距離の2乗
    pub fn dist_square(self, other: Point) -> i64 {
        (self - other).norm()
    }

    pub fn rot90(self) -> Self {
        Self::new(-self.y, self.x)
    }

    // 偏角ソート用の比較関数
    pub fn polar_cmp(&self, other: &Self) -> Ordering {
        let qa = half_plane(self);
        let qb = half_plane(other);

        if qa != qb {
            return qa.cmp(&qb);
        }

        let cross = self.cross(*other);
        if cross > 0 {
            Ordering::Less
        } else if cross < 0 {
            Ordering::Greater
        } else {
            self.norm().cmp(&other.norm())
        }
    }
}

fn half_plane(p: &Point) -> i8 {
    if p.x == 0 && p.y == 0 {
        0
    } else if p.y > 0 || (p.y == 0 && p.x > 0) {
        1
    } else {
        2
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// --- 演算子オーバーロード (省略せずにそのまま利用) ---
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y)
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y)
    }
}
impl Mul for Point {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(self.x * r.x - self.y * r.y, self.x * r.y + self.y * r.x)
    }
}
impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, r: i64) -> Self {
        Self::new(self.x * r, self.y * r)
    }
}
impl Div<i64> for Point {
    type Output = Self;
    fn div(self, r: i64) -> Self {
        Self::new(self.x / r, self.y / r)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, r: Self) {
        *self = *self + r;
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, r: Self) {
        *self = *self - r;
    }
}
impl MulAssign for Point {
    fn mul_assign(&mut self, r: Self) {
        *self = *self * r;
    }
}
impl MulAssign<i64> for Point {
    fn mul_assign(&mut self, r: i64) {
        *self = *self * r;
    }
}
impl DivAssign<i64> for Point {
    fn div_assign(&mut self, r: i64) {
        *self = *self / r;
    }
}

// 直線と線分を型として分離
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}
impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Circle {
    pub p: Point,
    pub r: i64,
}
impl Circle {
    pub fn new(p: Point, r: i64) -> Self {
        Self { p, r }
    }
}

//------------------------------//
// 交差判定・位置関係
//------------------------------//

pub fn ccw(a: Point, b: Point, c: Point) -> i32 {
    let cross = (b - a).cross(c - a);
    if cross > 0 {
        return 1;
    }
    if cross < 0 {
        return -1;
    }
    let dot = (b - a).dot(c - a);
    if dot < 0 {
        return 2;
    }
    if (b - a).norm() < (c - a).norm() {
        return -2;
    }
    0
}

pub fn is_orthogonal(v1: Point, v2: Point) -> bool {
    v1.dot(v2) == 0
}
pub fn is_parallel(v1: Point, v2: Point) -> bool {
    v1.cross(v2) == 0
}

pub fn is_inter_pl(p: Point, l: Line) -> bool {
    is_parallel(l.b - l.a, p - l.a)
}

pub fn is_inter_ps(p: Point, s: Segment) -> bool {
    ccw(s.a, s.b, p) == 0
}

pub fn is_inter_ll(l: Line, m: Line) -> bool {
    !is_parallel(l.b - l.a, m.b - m.a) || is_parallel(l.b - l.a, m.a - l.a)
}

pub fn is_inter_ss(s: Segment, t: Segment) -> bool {
    if s.a == s.b {
        return is_inter_ps(s.a, t);
    }
    if t.a == t.b {
        return is_inter_ps(t.a, s);
    }
    ccw(s.a, s.b, t.a) * ccw(s.a, s.b, t.b) <= 0 && ccw(t.a, t.b, s.a) * ccw(t.a, t.b, s.b) <= 0
}

//------------------------------//
// 応用アルゴリズム
//------------------------------//

/// 多角形の面積の2倍 (反時計回りに頂点が与えられていること)
/// 浮動小数点数を使わずに面積の大小を比較する際に有用
pub fn polygon_area_twice(poly: &[Point]) -> i64 {
    let mut area = 0;
    let n = poly.len();
    for i in 0..n {
        area += poly[i].cross(poly[(i + 1) % n]);
    }
    area.abs()
}

/// 凸包 (Monotone Chain)
pub fn convex_hull(mut pts: Vec<Point>) -> Vec<Point> {
    pts.sort_unstable(); // 辞書式順序でのソート（PartialOrd/Ord を Derive しているため動作する）
    pts.dedup();
    if pts.len() <= 2 {
        return pts;
    }
    let mut res = Vec::new();
    // 下側凸包
    for &p in &pts {
        while res.len() >= 2 && ccw(res[res.len() - 2], res[res.len() - 1], p) == -1 {
            res.pop();
        }
        res.push(p);
    }
    // 上側凸包
    let t = res.len() + 1;
    for &p in pts.iter().rev().skip(1) {
        while res.len() >= t && ccw(res[res.len() - 2], res[res.len() - 1], p) == -1 {
            res.pop();
        }
        res.push(p);
    }
    res.pop();
    res
}
