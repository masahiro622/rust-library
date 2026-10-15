use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

//------------------------------//
// 基本要素 (点, 線分, 円)
//------------------------------//

// basic settings
pub const INF: f64 = (1u64 << 60) as f64;
pub const EPS: f64 = 1e-10;

pub fn torad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn todeg(ang: f64) -> f64 {
    ang * 180.0 / PI
}

// Point or Vector
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // constructor
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // various functions
    pub fn conj(self) -> Self {
        Self::new(self.x, -self.y)
    }

    pub fn dot(self, r: Point) -> f64 {
        self.x * r.x + self.y * r.y
    }

    pub fn cross(self, r: Point) -> f64 {
        self.x * r.y - self.y * r.x
    }

    // ノルム (ベクトルの長さの2乗)
    pub fn norm(self) -> f64 {
        self.dot(self)
    }

    // 絶対値 (ベクトルの長さ)
    pub fn abs(self) -> f64 {
        self.x.hypot(self.y)
    }

    // 角度 (atan2の値を0~2πの範囲に変換)
    pub fn amp(self) -> f64 {
        let mut res = self.y.atan2(self.x);
        if res < 0.0 {
            res += PI * 2.0;
        }
        res
    }

    pub fn eq_eps(self, r: Point) -> bool {
        (self - r).abs() <= EPS
    }

    pub fn rot90(self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn rot(self, ang: f64) -> Self {
        let (sin_a, cos_a) = ang.sin_cos();
        Self::new(
            cos_a * self.x - sin_a * self.y,
            sin_a * self.x + cos_a * self.y,
        )
    }
}

// 点がどの領域にいるかを判定する関数
// 0: 原点
// 1: 上半平面 (y > 0 または (y == 0 かつ x > 0)) -> 角度 0 ~ π未満
// 2: 下半平面 -> 角度 π ~ 2π未満
fn half_plane(p: &Point) -> i8 {
    if p.x == 0.0 && p.y == 0.0 {
        0
    } else if p.y > 0.0 || (p.y == 0.0 && p.x > 0.0) {
        1
    } else {
        2
    }
}

fn sort_points_by_argument(points: &mut [Point]) {
    points.sort_by(|a, b| {
        let qa = half_plane(a);
        let qb = half_plane(b);

        // 1. 属している領域が違うなら、領域番号でソート
        if qa != qb {
            return qa.cmp(&qb);
        }

        // 2. 同じ領域にいる場合は、外積で反時計回りかどうかを判定
        // 外積 (a × b) > 0 ならば、b は a より反時計回り（偏角が大きい）
        let cross = a.cross(*b);
        if cross > 0.0 {
            std::cmp::Ordering::Less
        } else if cross < 0.0 {
            std::cmp::Ordering::Greater
        } else {
            // 3. 角度が同じ（同一直線上）なら距離で比較
            a.norm().total_cmp(&b.norm())
        }
    });
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// arithmetic operators
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

// 複素数としての乗算 (Point * Point)
impl Mul for Point {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(self.x * r.x - self.y * r.y, self.x * r.y + self.y * r.x)
    }
}

// 複素数としての除算 (Point / Point)
impl Div for Point {
    type Output = Self;
    fn div(self, r: Self) -> Self {
        self * r.conj() / r.norm()
    }
}

// スカラー倍 (Point * f64)
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, r: f64) -> Self {
        Self::new(self.x * r, self.y * r)
    }
}

// スカラー除算 (Point / f64)
impl Div<f64> for Point {
    type Output = Self;
    fn div(self, r: f64) -> Self {
        Self::new(self.x / r, self.y / r)
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

impl DivAssign for Point {
    fn div_assign(&mut self, r: Self) {
        *self = *self / r;
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, r: f64) {
        *self = *self * r;
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, r: f64) {
        *self = *self / r;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.a, self.b)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Circle {
    pub p: Point,
    pub r: f64,
}

impl Circle {
    pub fn new(p: Point, r: f64) -> Self {
        Self { p, r }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.p.x, self.p.y, self.r)
    }
}

//------------------------------//
// 円や直線の交差判定, 距離
//------------------------------//

/*
ccw (Counter-Clockwise) を用いている

P: Point
L: Line (直線)
S: Segment (線分。データ型としてはLineを使う)

distance_pl は、「点」と「直線」の距離
distance_ps は、「点」と「線分」の距離
*/

/// 点 c が有向線分 a->b に対してどの位置にあるか
pub fn ccw_for_dis(a: Point, b: Point, c: Point) -> i32 {
    let cross = (b - a).cross(c - a);
    if cross > EPS {
        return 1;
    } // 反時計回り (ccw)
    if cross < -EPS {
        return -1;
    } // 時計回り (cw)

    let dot = (b - a).dot(c - a);
    if dot < -EPS {
        return 2;
    } // 直線上、a より後ろ (online back)

    if (b - a).norm() < (c - a).norm() - EPS {
        return -2; // 直線上、b より前 (online front)
    }

    0 // 線分 a-b 上 (on segment)
}

/// 直線 l への点 p の射影（垂線の足）
pub fn proj(p: Point, l: Line) -> Point {
    if l.a.eq_eps(l.b) {
        return l.a; // 直線が点の場合はその点を返す
    }
    let t = (p - l.a).dot(l.b - l.a) / (l.b - l.a).norm();
    l.a + (l.b - l.a) * t
}

/// 直線 l を軸とした点 p の線対称な点
pub fn refl(p: Point, l: Line) -> Point {
    p + (proj(p, l) - p) * 2.0
}

/// 点 p が直線 l 上にあるか
pub fn is_inter_pl(p: Point, l: Line) -> bool {
    (p - proj(p, l)).abs() < EPS
}

/// 点 p が線分 s 上にあるか
pub fn is_inter_ps(p: Point, s: Line) -> bool {
    ccw_for_dis(s.a, s.b, p) == 0
}

/// 直線 l と直線 m が交差するか（平行でない、または同一の直線）
pub fn is_inter_ll(l: Line, m: Line) -> bool {
    (l.b - l.a).cross(m.b - m.a).abs() > EPS || (l.b - l.a).cross(m.a - l.a).abs() < EPS
}

/// 線分 s と線分 t が交差するか
pub fn is_inter_ss(s: Line, t: Line) -> bool {
    if s.a.eq_eps(s.b) {
        return is_inter_ps(s.a, t);
    }
    if t.a.eq_eps(t.b) {
        return is_inter_ps(t.a, s);
    }

    ccw_for_dis(s.a, s.b, t.a) * ccw_for_dis(s.a, s.b, t.b) <= 0
        && ccw_for_dis(t.a, t.b, s.a) * ccw_for_dis(t.a, t.b, s.b) <= 0
}

/// 点 p と直線 l の距離
pub fn distance_pl(p: Point, l: Line) -> f64 {
    (p - proj(p, l)).abs()
}

/// 点 p と線分 s の距離
pub fn distance_ps(p: Point, s: Line) -> f64 {
    let h = proj(p, s);
    if is_inter_ps(h, s) {
        return (p - h).abs();
    }
    // 垂線の足が線分外にある場合は、端点との距離の近い方を採用
    (p - s.a).abs().min((p - s.b).abs())
}

/// 直線 l と直線 m の距離
pub fn distance_ll(l: Line, m: Line) -> f64 {
    if is_inter_ll(l, m) {
        return 0.0;
    }
    distance_pl(m.a, l)
}

/// 線分 s と線分 t の距離
pub fn distance_ss(s: Line, t: Line) -> f64 {
    if is_inter_ss(s, t) {
        return 0.0;
    }

    let d_s_t = distance_ps(s.a, t).min(distance_ps(s.b, t));
    let d_t_s = distance_ps(t.a, s).min(distance_ps(t.b, s));

    d_s_t.min(d_t_s)
}
