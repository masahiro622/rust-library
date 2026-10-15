#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

fn cross(a: Point, b: Point, c: Point) -> isize {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let c1 = cross(a, b, p);
    let c2 = cross(b, c, p);
    let c3 = cross(c, a, p);

    // 全て正 or 全て負 なら true
    (c1 > 0 && c2 > 0 && c3 > 0) || (c1 < 0 && c2 < 0 && c3 < 0)

    // 外周を含む場合
    // (c1 >= 0 && c2 >= 0 && c3 >= 0) || (c1 <= 0 && c2 <= 0 && c3 <= 0)
}
