/// ax^2 + bx + c = 0 の整数解を返す
///
/// # 戻り値
/// - 整数解が存在する場合: Some((解1, 解2)) ※ 解1 <= 解2
/// - 整数解が存在しない場合: None
fn solve_quadratic_integer(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    if a == 0 {
        // a=0 (一次方程式 bx + c = 0) の場合
        if b == 0 {
            return None; // 解なし、または不定
        }
        if c % b == 0 {
            let x = -c / b;
            return Some((x, x));
        } else {
            return None;
        }
    }

    // オーバーフローを防ぐため i128 にキャスト
    let a = a as i128;
    let b = b as i128;
    let c = c as i128;

    // 判別式 D = b^2 - 4ac
    let d = b.pow(2) - 4 * a * c;

    if d < 0 {
        return None;
    }

    // 整数平方根の計算 (Rust 1.67以降で利用可能)
    let sqrt_d = d.isqrt();

    // Dが完全平方数かチェック
    if sqrt_d * sqrt_d != d {
        return None;
    }

    // 解の公式の分子: -b ± √D
    let num1 = -b - sqrt_d;
    let num2 = -b + sqrt_d;
    let den = 2 * a;

    // 割り切れるかチェック
    if num1 % den != 0 || num2 % den != 0 {
        return None;
    }

    let x1 = (num1 / den) as i64;
    let x2 = (num2 / den) as i64;

    // 小さい順に並べて返す
    if x1 < x2 {
        Some((x1, x2))
    } else {
        Some((x2, x1))
    }
}
