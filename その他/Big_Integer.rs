// 大きな整数の加算と乗算を行うモジュール
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

fn compare_abs(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    a.cmp(b)
}

fn add_abs(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(a.len().max(b.len()) + 1);
    let mut carry = 0;
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    while i >= 0 || j >= 0 || carry > 0 {
        let x = if i >= 0 {
            (a[i as usize] - b'0') as i32
        } else {
            0
        };
        let y = if j >= 0 {
            (b[j as usize] - b'0') as i32
        } else {
            0
        };
        let sum = x + y + carry;
        res.push((sum % 10) as u8 + b'0');
        carry = sum / 10;
        i -= 1;
        j -= 1;
    }
    res.reverse();
    res
}

fn sub_abs(a: &[u8], b: &[u8]) -> Vec<u8> {
    // assume a >= b
    let mut res = Vec::with_capacity(a.len());
    let mut borrow = 0;
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    while i >= 0 {
        let mut x = (a[i as usize] - b'0') as i32 - borrow;
        let y = if j >= 0 {
            (b[j as usize] - b'0') as i32
        } else {
            0
        };
        x -= y;
        if x < 0 {
            x += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        res.push(x as u8 + b'0');
        i -= 1;
        j -= 1;
    }
    while res.len() > 1 && res.last() == Some(&b'0') {
        res.pop();
    }
    res.reverse();
    res
}

fn add_signed(a: &str, b: &str) -> String {
    let (sign_a, digits_a) = if a.starts_with('-') {
        (false, &a[1..])
    } else {
        (true, a)
    };
    let (sign_b, digits_b) = if b.starts_with('-') {
        (false, &b[1..])
    } else {
        (true, b)
    };
    let da = digits_a.as_bytes();
    let db = digits_b.as_bytes();

    let result = if sign_a == sign_b {
        // same sign: just add
        let mut r = add_abs(da, db);
        if !sign_a {
            r.insert(0, b'-');
        }
        r
    } else {
        // different sign: subtract smaller from larger
        match compare_abs(da, db) {
            std::cmp::Ordering::Greater => {
                let mut r = sub_abs(da, db);
                if !sign_a {
                    r.insert(0, b'-');
                }
                r
            }
            std::cmp::Ordering::Less => {
                let mut r = sub_abs(db, da);
                if !sign_b {
                    r.insert(0, b'-');
                }
                r
            }
            std::cmp::Ordering::Equal => b"0".to_vec(),
        }
    };

    String::from_utf8(result).unwrap()
}

fn mul_bigint(a: &str, b: &str) -> String {
    // 符号を抽出
    let (sign_a, abs_a) = extract_sign_and_abs(a);
    let (sign_b, abs_b) = extract_sign_and_abs(b);

    // 結果の符号を決定
    let result_sign = sign_a * sign_b;

    // 絶対値で計算
    let a_digits: Vec<i64> = abs_a.bytes().rev().map(|x| (x - b'0') as i64).collect();
    let b_digits: Vec<i64> = abs_b.bytes().rev().map(|x| (x - b'0') as i64).collect();

    let mut res = convolution(&a_digits, &b_digits);

    // 繰り上がり処理
    let mut carry = 0;
    for i in 0..res.len() {
        res[i] += carry;
        carry = res[i] / 10;
        res[i] %= 10;
    }
    while carry > 0 {
        res.push(carry % 10);
        carry /= 10;
    }

    // 結果を文字列に変換
    res.reverse();
    let mut result = res
        .iter()
        .map(|&x| (x as u8 + b'0') as char)
        .collect::<String>();

    // 先頭の0を除去（ただし"0"は残す）
    while result.len() > 1 && result.starts_with('0') {
        result.remove(0);
    }

    // 符号を付加（結果が0でない場合のみ負号を付ける）
    if result_sign < 0 && result != "0" {
        result = format!("-{}", result);
    }

    result
}

fn extract_sign_and_abs(s: &str) -> (i32, &str) {
    if s.starts_with('-') {
        (-1, &s[1..])
    } else if s.starts_with('+') {
        (1, &s[1..])
    } else {
        (1, s)
    }
}
// 形式的冪級数
// convolution (O(n log n))

const MOD: i64 = 998244353;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % MOD;
        }
        exp >>= 1;
        base = (base * base) % MOD;
    }
    result
}

fn mod_inv(a: i64) -> i64 {
    mod_pow(a, MOD - 2)
}

// より効率的なNTT実装
fn ntt(a: &mut [i64], inverse: bool) {
    let n = a.len();
    assert!(n.is_power_of_two());

    // ビット反転（キャッシュ効率重視）
    let log_n = n.trailing_zeros();
    for i in 0..n {
        let j = reverse_bits(i, log_n);
        if i < j {
            a.swap(i, j);
        }
    }

    // 原始根のべき乗テーブルを事前計算
    let mut wn_table = vec![1i64; n / 2];
    let base_w = if inverse {
        mod_pow(3, MOD - 1 - (MOD - 1) / n as i64)
    } else {
        mod_pow(3, (MOD - 1) / n as i64)
    };

    for i in 1..n / 2 {
        wn_table[i] = (wn_table[i - 1] * base_w) % MOD;
    }

    let mut len = 2;
    while len <= n {
        let step = n / len;

        // ループアンローリング（4つずつ処理）
        for i in (0..n).step_by(len) {
            let half_len = len / 2;

            for j in (0..half_len).step_by(4) {
                // 4つ同時処理
                for k in 0..4.min(half_len - j) {
                    let idx = j + k;
                    let u = a[i + idx];
                    let v = (a[i + idx + half_len] * wn_table[idx * step]) % MOD;

                    a[i + idx] = if u + v >= MOD { u + v - MOD } else { u + v };
                    a[i + idx + half_len] = if u >= v { u - v } else { u - v + MOD };
                }
            }
        }
        len <<= 1;
    }

    if inverse {
        let n_inv = mod_inv(n as i64);
        for val in a.iter_mut() {
            *val = (*val * n_inv) % MOD;
        }
    }
}

// ビット反転のヘルパー関数
fn reverse_bits(mut num: usize, bits: u32) -> usize {
    let mut result = 0;
    for _ in 0..bits {
        result = (result << 1) | (num & 1);
        num >>= 1;
    }
    result
}

// 高速化された畳み込み関数（メモリ使用量削減）
fn convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let n = a.len() + b.len() - 1;
    let size = n.next_power_of_two();

    // メモリ効率を向上させるため、片方を再利用
    let mut fa = Vec::with_capacity(size);
    fa.extend_from_slice(a);
    fa.resize(size, 0);

    let mut fb = Vec::with_capacity(size);
    fb.extend_from_slice(b);
    fb.resize(size, 0);

    ntt(&mut fa, false);
    ntt(&mut fb, false);

    // インプレースで乗算
    for i in 0..size {
        fa[i] = (fa[i] * fb[i]) % MOD;
    }
    // fb は不要になったのでドロップ
    drop(fb);

    ntt(&mut fa, true);
    fa.truncate(n);
    fa
}
