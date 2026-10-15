/// 三分探索（最小値）
/// 関数 `f` の最小値を実数範囲 [left, right] 内で探索（精度: eps）
pub fn ternary_search<F>(mut left: f64, mut right: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut limit = 0; // ループ制限
    while right - left > eps && limit < 300 {
        let c1 = left + (right - left) / 3.0;
        let c2 = right - (right - left) / 3.0;
        // 最大値探索にする場合は `<` を `>` に変更する。
        if f(c1) < f(c2) {
            right = c2;
        } else {
            left = c1;
        }
        limit += 1;
    }
    (left + right) / 2.0
}

/// 整数三分探索（最小値の位置）
/// 関数 f: i64 → i64 が単峰性を持つと仮定
pub fn ternary_search_int<F>(mut left: i64, mut right: i64, f: F) -> i64
where
    F: Fn(i64) -> i64,
{
    while right - left > 3 {
        let c1 = left + (right - left) / 3;
        let c2 = right - (right - left) / 3;
        // 最大値探索にする場合は `<` を `>` に変更する。
        if f(c1) < f(c2) {
            right = c2;
        } else {
            left = c1;
        }
    }

    let mut best = left;
    for i in left..=right {
        // 最大値探索にする場合は `<` を `>` に変更する。
        if f(i) < f(best) {
            best = i;
        }
    }
    best
}

fn main() {
    // f(x) = (x - 2)^2 + 1 の最小値を探す（x ∈ [0, 4]）
    let f = |x: f64| (x - 2.0).powi(2) + 1.0;
    let result = ternary_search(0.0, 4.0, f, 1e-9);
    println!("最小値の位置: {:.9}", result);
    println!("最小値: {:.9}", f(result));
}
