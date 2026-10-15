pub fn convex_hull(pts: &[Point]) -> Vec<usize> {
    let n = pts.len();
    if n == 0 {
        return vec![];
    }

    // 1. 辞書式順序でソート
    let mut ids: Vec<usize> = (0..n).collect();
    ids.sort_unstable_by(|&i, &j| pts[i].cmp(&pts[j]));

    ids.dedup_by(|&mut i, &mut j| pts[i] == pts[j]);

    // 重複を除いた後の頂点数が2以下の場合はそのまま返す
    if ids.len() <= 2 {
        return ids;
    }

    let mut res = Vec::new();

    // 判定関数: a -> b -> c と進むとき、厳密に反時計回り (左折) かどうか
    // 外積が 0 以下のとき (一直線上、または時計回り) は false を返す
    let is_strictly_ccw =
        |a: usize, b: usize, c: usize| -> bool { (pts[b] - pts[a]).cross(pts[c] - pts[a]) > 0 };

    // 下側凸包の構築
    for &i in &ids {
        while res.len() >= 2 {
            let len = res.len();
            let a = res[len - 2];
            let b = res[len - 1];
            if !is_strictly_ccw(a, b, i) {
                res.pop(); // 辺上の点や、内側に凹む点を取り除く
            } else {
                break;
            }
        }
        res.push(i);
    }

    // 上側凸包の構築
    let t = res.len() + 1;
    for &i in ids.iter().rev().skip(1) {
        while res.len() >= t {
            let len = res.len();
            let a = res[len - 2];
            let b = res[len - 1];
            if !is_strictly_ccw(a, b, i) {
                res.pop();
            } else {
                break;
            }
        }
        res.push(i);
    }

    res.pop(); // 始点と終点が重複するため削除

    // もし入力のすべての点が同一直線上にあった場合、
    // 上のループを経ると res には端の2点のみが残り、|S'| = 2 として正しく返されます
    res
}

pub fn furthest_pair(pts: &[Point]) -> (usize, usize) {
    let n_pts = pts.len();

    // 要素数が2未満の場合はペアが作れないためフォールバック
    if n_pts <= 1 {
        return (0, 0);
    }

    let hull = convex_hull(pts);
    let n = hull.len();

    // 1. 同一点の考慮: すべての点が同一座標の場合 (最大距離 0)
    // 重複排除された凸包の頂点数は1になるため、強制的に (0, 1) を返すことで
    // 「相異なるインデックス」の条件を満たす。
    if n == 1 {
        return (0, 1);
    }

    // 2. 凸包が潰れて線分になる場合 (同一直線上の点のみの場合)
    if n == 2 {
        return (hull[0], hull[1]);
    }

    // キャリパー法本体
    let mut max_d: i64 = -1;
    let mut res = (hull[0], hull[0]);
    let mut j = 1;

    for i in 0..n {
        let ni = (i + 1) % n;
        let edge = pts[hull[ni]] - pts[hull[i]];

        while {
            let nj = (j + 1) % n;
            edge.cross(pts[hull[nj]] - pts[hull[i]]) > edge.cross(pts[hull[j]] - pts[hull[i]])
        } {
            j = (j + 1) % n;
        }

        let d1 = pts[hull[i]].dist_square(pts[hull[j]]);
        if d1 > max_d {
            max_d = d1;
            res = (hull[i], hull[j]);
        }

        let d2 = pts[hull[ni]].dist_square(pts[hull[j]]);
        if d2 > max_d {
            max_d = d2;
            res = (hull[ni], hull[j]);
        }
    }

    res
}
