// 依存: geometric_base.rs (Point, Circle)

fn circle_from_1(a: Point) -> Circle {
    Circle::new(a, 0.0)
}

fn circle_from_2(a: Point, b: Point) -> Circle {
    let center = (a + b) / 2.0;
    Circle::new(center, (center - a).abs())
}

fn circle_from_3(a: Point, b: Point, c: Point) -> Circle {
    let x1 = a.x;
    let y1 = a.y;
    let x2 = b.x;
    let y2 = b.y;
    let x3 = c.x;
    let y3 = c.y;
    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    let eps = 1e-12;
    if d.abs() < eps {
        // ほぼ共線なら最大距離の2点の円にする
        let d12 = (a - b).norm();
        let d23 = (b - c).norm();
        let d31 = (c - a).norm();
        if d12 >= d23 && d12 >= d31 {
            return circle_from_2(a, b);
        } else if d23 >= d12 && d23 >= d31 {
            return circle_from_2(b, c);
        } else {
            return circle_from_2(c, a);
        }
    }
    let s1 = x1 * x1 + y1 * y1;
    let s2 = x2 * x2 + y2 * y2;
    let s3 = x3 * x3 + y3 * y3;
    let ux = (s1 * (y2 - y3) + s2 * (y3 - y1) + s3 * (y1 - y2)) / d;
    let uy = (s1 * (x3 - x2) + s2 * (x1 - x3) + s3 * (x2 - x1)) / d;
    let center = Point::new(ux, uy);
    Circle::new(center, (center - a).abs())
}

fn is_outside(circle: &Circle, p: Point) -> bool {
    let eps: f64 = 1e-7f64;
    let r2 = circle.r * circle.r;
    (circle.p - p).norm() > r2 + eps.max(1e-12f64 * r2.abs())
}

// ===== 自作乱数生成器 (LCG) =====
struct MyRng {
    state: u64,
}
impl MyRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        // LCG: X_{n+1} = a*X_n + c (mod 2^64)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let span = (range.end - range.start) as u64;
        (self.next_u64() % span) as usize + range.start
    }
}
fn shuffle<T>(v: &mut [T], rng: &mut MyRng) {
    let n = v.len();
    for i in (1..n).rev() {
        let j = rng.gen_range(0..i + 1);
        v.swap(i, j);
    }
}

/// 最小包含円を求める (Welzlのアルゴリズム、期待計算量 O(n))
/// points が空の場合は None を返す
pub fn minimum_enclosing_circle(points: &[Point]) -> Option<Circle> {
    let n = points.len();
    if n == 0 {
        return None;
    }
    let mut pts = points.to_vec();

    // シャッフル (期待計算量を保証)
    let mut rng = MyRng::new(88172645463325252); // 適当なシード
    shuffle(&mut pts, &mut rng);

    // Welzl
    let mut circ = circle_from_1(pts[0]);
    for i in 1..n {
        let pi = pts[i];
        if !is_outside(&circ, pi) {
            continue;
        }
        circ = circle_from_1(pi);
        for j in 0..i {
            let pj = pts[j];
            if !is_outside(&circ, pj) {
                continue;
            }
            circ = circle_from_2(pi, pj);
            for k in 0..j {
                let pk = pts[k];
                if !is_outside(&circ, pk) {
                    continue;
                }
                circ = circle_from_3(pi, pj, pk);
            }
        }
    }
    Some(circ)
}
