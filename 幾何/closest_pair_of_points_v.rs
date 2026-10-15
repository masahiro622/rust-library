pub fn closest_pair(points: Vec<Point>) -> (i64, (usize, usize)) {
    // 1. 内部計算用に (Point, 元のインデックス) の配列を生成
    let mut tagged_points: Vec<(Point, usize)> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    tagged_points.sort_unstable_by_key(|&(p, _)| p);

    // 2. 実際の再帰処理は内部関数で行う
    closest_pair_internal(&mut tagged_points)
}

/// 実際の分割統治を行う内部関数
fn closest_pair_internal(points: &mut [(Point, usize)]) -> (i64, (usize, usize)) {
    let n = points.len();
    if n <= 1 {
        return (i64::MAX, (0, 0));
    }

    let mid = n / 2;
    // タプルの .0 が Point、 .1 が元のインデックス
    let midx = points[mid].0.x;

    let (d1, p1) = closest_pair_internal(&mut points[..mid]);
    let (d2, p2) = closest_pair_internal(&mut points[mid..]);

    let (mut d, mut best_pair) = if d1 < d2 { (d1, p1) } else { (d2, p2) };
    // merge by y
    let mut aux = vec![(Point::new(0, 0), 0); n];
    {
        let mut i = 0;
        let mut j = mid;
        let mut k = 0;
        while i < mid && j < n {
            if points[i].0.y <= points[j].0.y {
                aux[k] = points[i];
                i += 1;
            } else {
                aux[k] = points[j];
                j += 1;
            }
            k += 1;
        }
        while i < mid {
            aux[k] = points[i];
            i += 1;
            k += 1;
        }
        while j < n {
            aux[k] = points[j];
            j += 1;
            k += 1;
        }
        points.copy_from_slice(&aux[..n]);
    }

    // strip: |x - midx| < sqrt(d)
    let mut strip = Vec::new();
    for &p in points.iter() {
        let dx = (p.0.x - midx).abs();
        // オーバーフローを避けるため乗算チェック
        if dx.saturating_mul(dx) < d {
            strip.push(p);
        }
    }

    // 比較対象は高々 6~7 個に限定できる
    for i in 0..strip.len() {
        for j in i + 1..strip.len().min(i + 8) {
            let pd = strip[i].0.dist_square(strip[j].0);
            if pd < d {
                d = pd;
                // 配列の position 検索をなくし、保持していた元のインデックスを直接返す
                best_pair = (strip[i].1, strip[j].1);
            }
        }
    }

    (d, best_pair)
}
