use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

//------------------------------//
// 基本要素 (点, 線分, 平面, 球)
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
    pub z: f64,
}

impl Point {
    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // various functions
    pub fn dot(self, r: Point) -> f64 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }

    // 外積 (3Dでは結果もベクトルになる)
    pub fn cross(self, r: Point) -> Point {
        Self::new(
            self.y * r.z - self.z * r.y,
            self.z * r.x - self.x * r.z,
            self.x * r.y - self.y * r.x,
        )
    }

    // ノルム (ベクトルの長さの2乗)
    pub fn norm(self) -> f64 {
        self.dot(self)
    }

    // 絶対値 (ベクトルの長さ)
    pub fn abs(self) -> f64 {
        self.norm().sqrt()
    }

    // 単位ベクトル
    pub fn unit(self) -> Self {
        self / self.abs()
    }

    pub fn eq_eps(self, r: Point) -> bool {
        (self - r).abs() <= EPS
    }

    // 2ベクトルのなす角 (0 ~ π)
    pub fn angle(self, r: Point) -> f64 {
        (self.dot(r) / (self.abs() * r.abs())).clamp(-1.0, 1.0).acos()
    }

    // 原点を通り axis 方向を軸として ang だけ回転 (ロドリゲスの回転公式, axis は単位ベクトルでなくてよい)
    pub fn rot(self, axis: Point, ang: f64) -> Self {
        let n = axis.unit();
        let (sin_a, cos_a) = ang.sin_cos();
        self * cos_a + n.cross(self) * sin_a + n * (n.dot(self) * (1.0 - cos_a))
    }

    // x軸, y軸, z軸周りの回転 (よく使うのでショートカットとして用意)
    pub fn rot_x(self, ang: f64) -> Self {
        let (sin_a, cos_a) = ang.sin_cos();
        Self::new(self.x, cos_a * self.y - sin_a * self.z, sin_a * self.y + cos_a * self.z)
    }

    pub fn rot_y(self, ang: f64) -> Self {
        let (sin_a, cos_a) = ang.sin_cos();
        Self::new(cos_a * self.x + sin_a * self.z, self.y, -sin_a * self.x + cos_a * self.z)
    }

    pub fn rot_z(self, ang: f64) -> Self {
        let (sin_a, cos_a) = ang.sin_cos();
        Self::new(cos_a * self.x - sin_a * self.y, sin_a * self.x + cos_a * self.y, self.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// arithmetic operators
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}

// スカラー倍 (Point * f64)
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, r: f64) -> Self {
        Self::new(self.x * r, self.y * r, self.z * r)
    }
}

// スカラー除算 (Point / f64)
impl Div<f64> for Point {
    type Output = Self;
    fn div(self, r: f64) -> Self {
        Self::new(self.x / r, self.y / r, self.z / r)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, r: Self) {
        *self = *self - r;
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

// 平面 (点 p を通り、法線ベクトル n を持つ)
#[derive(Debug, Clone, Copy, Default)]
pub struct Plane {
    pub p: Point,
    pub n: Point,
}

impl Plane {
    pub fn new(p: Point, n: Point) -> Self {
        Self { p, n }
    }

    // 3点から平面を作る
    pub fn from_points(a: Point, b: Point, c: Point) -> Self {
        Self::new(a, (b - a).cross(c - a))
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{p: {}, n: {}}}", self.p, self.n)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere {
    pub p: Point,
    pub r: f64,
}

impl Sphere {
    pub fn new(p: Point, r: f64) -> Self {
        Self { p, r }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.p.x, self.p.y, self.p.z, self.r)
    }
}

//------------------------------//
// 直線や平面の交差判定, 距離
//------------------------------//

/*
P: Point
L: Line (直線)
S: Segment (線分。データ型としてはLineを使う)
Pl: Plane (平面)

distance_pl は、「点」と「直線」の距離
distance_ps は、「点」と「線分」の距離

2Dのccwに相当する向き判定は、3Dでは4点が張る四面体の符号付き体積 (orient3d) で行う。
*/

/// 4点 a, b, c, d が張る四面体の符号付き体積の6倍
/// 0に近ければ4点は同一平面上にある (共面判定に使う)
pub fn orient3d(a: Point, b: Point, c: Point, d: Point) -> f64 {
    (b - a).cross(c - a).dot(d - a)
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

/// 平面 pl への点 p の射影（垂線の足）
pub fn proj_plane(p: Point, pl: Plane) -> Point {
    let t = (p - pl.p).dot(pl.n) / pl.n.norm();
    p - pl.n * t
}

/// 平面 pl を軸とした点 p の面対称な点
pub fn refl_plane(p: Point, pl: Plane) -> Point {
    p + (proj_plane(p, pl) - p) * 2.0
}

/// 点 p が直線 l 上にあるか
pub fn is_inter_pl(p: Point, l: Line) -> bool {
    (p - proj(p, l)).abs() < EPS
}

/// 点 p が線分 s 上にあるか
pub fn is_inter_ps(p: Point, s: Line) -> bool {
    is_inter_pl(p, s) && (p - s.a).dot(p - s.b) < EPS
}

/// 点 p が平面 pl 上にあるか
pub fn is_inter_p_plane(p: Point, pl: Plane) -> bool {
    (p - proj_plane(p, pl)).abs() < EPS
}

/// 直線 l と直線 m が交差するか（共面かつ平行でない、または同一の直線）
pub fn is_inter_ll(l: Line, m: Line) -> bool {
    let d1 = l.b - l.a;
    let d2 = m.b - m.a;
    if d1.cross(d2).abs() < EPS {
        // 平行 → 同一直線上にあるかどうか
        return is_inter_pl(m.a, l);
    }
    // 平行でないとき、共面であれば1点で交わる
    orient3d(l.a, l.b, m.a, m.b).abs() < EPS
}

/// 線分 s と線分 t が交差するか
pub fn is_inter_ss(s: Line, t: Line) -> bool {
    if s.a.eq_eps(s.b) {
        return is_inter_ps(s.a, t);
    }
    if t.a.eq_eps(t.b) {
        return is_inter_ps(t.a, s);
    }

    let d1 = s.b - s.a;
    let d2 = t.b - t.a;

    if d1.cross(d2).abs() < EPS {
        // 平行 → 同一直線上にあるかどうかを確認し、パラメータの範囲が重なるか調べる
        if !is_inter_pl(t.a, s) {
            return false;
        }
        let denom = d1.norm();
        let ta = (t.a - s.a).dot(d1) / denom;
        let tb = (t.b - s.a).dot(d1) / denom;
        let (lo, hi) = (ta.min(tb), ta.max(tb));
        hi >= -EPS && lo <= 1.0 + EPS
    } else {
        // 平行でない → 共面でなければ (ねじれの位置なら) 交わらない
        if orient3d(s.a, s.b, t.a, t.b).abs() > EPS {
            return false;
        }
        // 共面な2直線の交点を媒介変数 u, v で求め、両方とも [0, 1] に収まるか判定
        let n = d1.cross(d2);
        let denom = n.norm();
        let u = (t.a - s.a).cross(d2).dot(n) / denom;
        let v = (t.a - s.a).cross(d1).dot(n) / denom;
        (-EPS..=1.0 + EPS).contains(&u) && (-EPS..=1.0 + EPS).contains(&v)
    }
}

/// 直線 l と平面 pl が交差するか
pub fn is_inter_l_plane(l: Line, pl: Plane) -> bool {
    let d = l.b - l.a;
    if d.dot(pl.n).abs() > EPS {
        return true; // 平面と平行でなければ必ず1点で交わる
    }
    is_inter_p_plane(l.a, pl) // 平行なら、直線が平面に含まれているかどうか
}

/// 平面 pl1 と平面 pl2 が交差するか
pub fn is_inter_plane_plane(pl1: Plane, pl2: Plane) -> bool {
    if pl1.n.cross(pl2.n).abs() > EPS {
        return true; // 法線が平行でなければ必ず交線を持つ
    }
    is_inter_p_plane(pl2.p, pl1) // 法線が平行なら、同一平面かどうか
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

/// 点 p と平面 pl の距離
pub fn distance_p_plane(p: Point, pl: Plane) -> f64 {
    (p - pl.p).dot(pl.n).abs() / pl.n.abs()
}

/// 直線 l と直線 m の距離（ねじれの位置にある場合も対応）
pub fn distance_ll(l: Line, m: Line) -> f64 {
    if is_inter_ll(l, m) {
        return 0.0;
    }
    let d1 = l.b - l.a;
    let d2 = m.b - m.a;
    if d1.cross(d2).abs() < EPS {
        // 平行（かつ同一直線ではない）
        return distance_pl(m.a, l);
    }
    // ねじれの位置 → 共通垂線の長さ
    let n = d1.cross(d2);
    (m.a - l.a).dot(n).abs() / n.abs()
}

/// 線分 s と線分 t の距離
/// 2Dと違い3Dではねじれの位置の場合に最近点が両方とも内部になり得るため、
/// 端点との距離だけでは不十分。媒介変数を直接クランプして最近点を求める。
pub fn distance_ss(s: Line, t: Line) -> f64 {
    let d1 = s.b - s.a;
    let d2 = t.b - t.a;
    let r = s.a - t.a;

    let a = d1.norm();
    let e = d2.norm();
    let f = d2.dot(r);

    let (mut sc, mut tc);

    if a < EPS && e < EPS {
        return (s.a - t.a).abs();
    }
    if a < EPS {
        sc = 0.0;
        tc = (f / e).clamp(0.0, 1.0);
    } else {
        let c = d1.dot(r);
        if e < EPS {
            tc = 0.0;
            sc = (-c / a).clamp(0.0, 1.0);
        } else {
            let b = d1.dot(d2);
            let denom = a * e - b * b;

            sc = if denom.abs() > EPS {
                ((b * f - c * e) / denom).clamp(0.0, 1.0)
            } else {
                0.0
            };

            tc = (b * sc + f) / e;
            if tc < 0.0 {
                tc = 0.0;
                sc = (-c / a).clamp(0.0, 1.0);
            } else if tc > 1.0 {
                tc = 1.0;
                sc = ((b - c) / a).clamp(0.0, 1.0);
            }
        }
    }

    let p1 = s.a + d1 * sc;
    let p2 = t.a + d2 * tc;
    (p1 - p2).abs()
}

/// 直線 l と平面 pl の距離
pub fn distance_l_plane(l: Line, pl: Plane) -> f64 {
    if is_inter_l_plane(l, pl) {
        return 0.0;
    }
    distance_p_plane(l.a, pl)
}

/// 平面 pl1 と平面 pl2 の距離
pub fn distance_plane_plane(pl1: Plane, pl2: Plane) -> f64 {
    if is_inter_plane_plane(pl1, pl2) {
        return 0.0;
    }
    distance_p_plane(pl2.p, pl1)
}
