// (p,q,r,s) は (p+r)/(q+s) を表す

fn parse_decimal_to_fraction(s: &str) -> (i128, i128) {
    let mut num: i128 = 0;
    let mut den: i128 = 1;

    if let Some((int_part, frac_part)) = s.split_once('.') {
        let int_num = int_part.parse::<i128>().unwrap();
        for c in frac_part.bytes() {
            num = num * 10 + (c - b'0') as i128;
            den *= 10;
        }
        num += int_num * den;
    } else {
        num = s.parse::<i128>().unwrap();
    }

    let g = gcd_i128(num, den);
    (num / g, den / g)
}

// 分母と分子がともに n 以下の分数で x/y に最も近いものを、x/y より小さいものと大きいものの両方について求める
fn rational_approximation(n: i64, x: i64, y: i64) -> (i64, i64, i64, i64) {
    let (lp, lq, _, _) = search(
        |a, b| (a as i128) * (y as i128) <= (b as i128) * (x as i128),
        n,
    );

    let (_, _, rp, rq) = search(
        |a, b| (a as i128) * (y as i128) < (b as i128) * (x as i128),
        n,
    );

    (lp, lq, rp, rq)
}

fn child(d: i64, p: i64, q: i64, r: i64, s: i64, isleft: bool) -> (i64, i64, i64, i64) {
    if isleft {
        (p, q, r + d * p, s + d * q)
    } else {
        (p + r * d, q + s * d, r, s)
    }
}

fn parent(p: i64, q: i64, r: i64, s: i64) -> (i64, i64, i64, i64) {
    if (p, q, r, s) == (0, 1, 1, 0) {
        (0, 0, 0, 0)
    } else if p < r || q < s {
        (p, q, r - p, s - q)
    } else {
        (p - r, q - s, r, s)
    }
}

// 1/1 からp/q へのパスが右にpath[0]回、左にpath[1]回、右にpath[2]回、...と続くようなパスを表す
fn encode_path(mut p: i64, mut q: i64) -> Vec<i64> {
    let mut path: Vec<i64> = Vec::new();

    while q != 0 {
        path.push(p / q);
        let r = p % q;
        p = q;
        q = r;
    }

    if !path.is_empty() {
        if path.last() == Some(&1) {
            path.pop();
        } else {
            *path.last_mut().unwrap() -= 1;
        }
    }

    path
}

// 右にpath[0]回、左にpath[1]回、右にpath[2]回、...と続くようなパスを 1/1 からたどるときの分子と分母
fn decode_path(path: &[i64]) -> (i64, i64, i64, i64) {
    let mut p = 1;
    let mut q = 0;
    let mut r = 0;
    let mut s = 1;
    for i in 0..path.len() {
        let (np, nq, nr, ns) = child(path[i], p, q, r, s, i % 2 == 0);
        p = np;
        q = nq;
        r = nr;
        s = ns;
    }
    (p, q, r, s)
}

// p1/q1 と p2/q2 の最小共通祖先
fn lca(p1: i64, q1: i64, p2: i64, q2: i64) -> (i64, i64, i64, i64) {
    let path1 = encode_path(p1, q1);
    let path2 = encode_path(p2, q2);
    let mut f = 0;
    let mut g = 1;
    let mut h = 1;
    let mut k = 0;
    let n = path1.len().min(path2.len());
    for i in 0..n {
        (f, g, h, k) = child(path1[i].min(path2[i]), f, g, h, k, i % 2 == 1);
        if path1[i] != path2[i] {
            break;
        }
    }
    (f, g, h, k)
}

// p/q から d だけ上に遡った祖先
fn ancestor(d: i64, p: i64, q: i64) -> Option<(i64, i64, i64, i64)> {
    let path = encode_path(p, q);
    let mut f = 0;
    let mut g = 1;
    let mut h = 1;
    let mut k = 0;
    let mut d = d;
    for i in 0..path.len() {
        let c = d.min(path[i]);
        (f, g, h, k) = child(c, f, g, h, k, i % 2 == 1);
        d -= c;
        if d == 0 {
            break;
        }
    }
    if d == 0 {
        Some((f, g, h, k))
    } else {
        None
    }
}

// p/q の子孫が取りうる分数の範囲
fn range(p: i64, q: i64) -> (i64, i64, i64, i64) {
    let (up_p, up_q, low_p, low_q) = decode_path(&encode_path(p, q));
    (low_p, low_q, up_p, up_q)
}

fn search<F>(judge: F, max_value: i64) -> (i64, i64, i64, i64)
where
    F: Fn(i64, i64) -> bool,
{
    let mut p = 0;
    let mut q = 1;
    let mut r = 1;
    let mut s = 0;

    let judge_01 = judge(0, 1);
    let judge_10 = judge(1, 0);
    assert_ne!(judge_01, judge_10);

    while p + r <= max_value && q + s <= max_value {
        let judge_now = judge(p + r, q + s);
        let isleft = judge_now ^ judge_01;

        let maxd = if isleft {
            if p == 0 {
                (max_value - s) / q
            } else {
                ((max_value - r) / p).min((max_value - s) / q)
            }
        } else if s == 0 {
            (max_value - p) / r
        } else {
            ((max_value - q) / s).min((max_value - p) / r)
        };

        let mut dl = 1;
        let mut dr = 2;
        while dr <= maxd {
            let (np, nq, nr, ns) = child(dr, p, q, r, s, isleft);
            if judge(np + nr, nq + ns) != judge_now {
                break;
            }
            dl = dr;
            dr = dr.saturating_mul(2).min(maxd + 1);
        }

        while dr - dl > 1 {
            let dm = dl + (dr - dl) / 2;
            let (np, nq, nr, ns) = child(dm, p, q, r, s, isleft);
            if judge(np + nr, nq + ns) == judge_now {
                dl = dm;
            } else {
                dr = dm;
            }
        }

        (p, q, r, s) = child(dl, p, q, r, s, isleft);
    }

    (p, q, r, s)
}
