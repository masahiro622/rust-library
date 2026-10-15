struct S {
    min_val: i32,   // 区間の最小被覆数
    min_len: isize, // 最小被覆数をもつ区間の長さの合計
}

// データに対するモノイド
struct SMonoid;
impl Monoid for SMonoid {
    type S = S;
    fn identity() -> Self::S {
        S {
            min_val: i32::MAX, // 単位元は十分に大きな値
            min_len: 0,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.min_val < b.min_val {
            *a
        } else if a.min_val > b.min_val {
            *b
        } else {
            // 最小値が同じ場合は、その長さを足し合わせる
            S {
                min_val: a.min_val,
                min_len: a.min_len + b.min_len,
            }
        }
    }
}

// 遅延評価とデータ更新の作用を表すモノイド
struct AreaMapMonoid;
impl MapMonoid for AreaMapMonoid {
    type M = SMonoid;
    type F = i32; // 被覆数の増減値 (+1 or -1)

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if x.min_val == i32::MAX {
            *x
        } else {
            S {
                min_val: x.min_val + f,
                min_len: x.min_len,
            }
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

// 複数の矩形の合計面積を計算する関数
// rectsは半開区間 [l, r) x [d, u) で表される矩形のリスト
// rectsの形式: (l, d, r, u) = (左端x, 下端y, 右端x, 上端y)
fn calc_union_area(rects: &[(isize, isize, isize, isize)]) -> isize {
    let n = rects.len();
    if n == 0 {
        return 0;
    }

    // 1. y座標の座標圧縮
    let mut ys = Vec::with_capacity(n * 2);
    for &(_, d, _, u) in rects {
        ys.push(d);
        ys.push(u);
    }
    ys.sort_unstable();
    ys.dedup();

    // 2. イベントの作成とソート
    // (x座標, 変化量(+1 or -1), 下端y, 上端y)
    let mut events = Vec::with_capacity(n * 2);
    for &(l, d, r, u) in rects {
        events.push((l, 1, d, u));
        events.push((r, -1, d, u));
    }
    events.sort_unstable_by_key(|e| e.0);

    // 3. セグメント木の初期化
    let m = ys.len() - 1;
    let mut initial_leaves = Vec::with_capacity(m);
    for i in 0..m {
        initial_leaves.push(S {
            min_val: 0,
            min_len: ys[i + 1] - ys[i], // 基本区間の長さ
        });
    }

    let mut seg = LazySegtree::<AreaMapMonoid>::from(initial_leaves);
    let total_y_length: isize = ys.last().unwrap() - ys.first().unwrap();

    let mut area = 0isize;
    let mut prev_x = events[0].0;

    // 4. イベントの処理と面積の計算
    for (x, val, d, u) in events {
        // x座標が進んだ場合、これまでのx幅 × 現在の有効y長さを面積に加算
        if x > prev_x {
            let all = seg.all_prod();
            let active_y = if all.min_val == 0 {
                // 最小被覆数が0の場合、全体から被覆されていない長さを引く
                total_y_length - all.min_len
            } else {
                // 最小被覆数が1以上なら、全区間が被覆されている
                total_y_length
            };
            area += (x - prev_x) * active_y;
            prev_x = x;
        }

        // 対象のy区間に対して被覆数を更新 (座標圧縮されたインデックスを取得)
        let d_idx = ys.binary_search(&d).unwrap();
        let u_idx = ys.binary_search(&u).unwrap();
        seg.apply_range(d_idx..u_idx, val);
    }

    area
}
