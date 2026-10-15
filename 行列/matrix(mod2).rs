// Vec<u8>を文字列に変換するユーティリティ関数
fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| bit_to_char(b)).collect()
}

// 文字列(0/1)で受け取る行列を内部でビットパッキングして扱う
// 例: ["0101", "1110", ...]

fn validate_binary_matrix(rows: &[String]) {
    if rows.is_empty() {
        return;
    }
    let m = rows[0].len();
    for row in rows {
        assert!(row.len() == m, "All rows must have the same length.");
        assert!(
            row.as_bytes().iter().all(|&c| c == b'0' || c == b'1'),
            "Matrix rows must contain only '0' or '1'."
        );
    }
}

fn bit_to_char(b: u8) -> char {
    if b == 0 {
        '0'
    } else {
        '1'
    }
}

#[inline]
fn packed_word_len(bits: usize) -> usize {
    (bits + 63) / 64
}

fn pack_binary_rows(rows: &[String]) -> Vec<Vec<u64>> {
    validate_binary_matrix(rows);
    if rows.is_empty() {
        return vec![];
    }

    let n = rows.len();
    let m = rows[0].len();
    let w = packed_word_len(m);
    let mut packed = vec![vec![0u64; w]; n];

    for i in 0..n {
        let bytes = rows[i].as_bytes();
        for j in 0..m {
            if bytes[j] == b'1' {
                packed[i][j / 64] |= 1u64 << (j % 64);
            }
        }
    }

    packed
}

fn pack_binary_cols(rows: &[String]) -> Vec<Vec<u64>> {
    validate_binary_matrix(rows);
    if rows.is_empty() {
        return vec![];
    }

    let m = rows.len();
    let k = rows[0].len();
    let w = packed_word_len(m);
    let mut packed = vec![vec![0u64; w]; k];

    for i in 0..m {
        let bytes = rows[i].as_bytes();
        for j in 0..k {
            if bytes[j] == b'1' {
                packed[j][i / 64] |= 1u64 << (i % 64);
            }
        }
    }

    packed
}

fn pack_binary_vector(bits: &str) -> Vec<u8> {
    assert!(
        bits.as_bytes().iter().all(|&c| c == b'0' || c == b'1'),
        "rhs must contain only '0' or '1'."
    );
    bits.as_bytes().iter().map(|&c| c - b'0').collect()
}

/// A(n×m), B(m×k) を mod 2 で掛ける。
/// 入出力はすべて文字列配列。
pub fn matrix_multiply_mod2_bitpacked_strings(a: &[String], b: &[String]) -> Vec<String> {
    validate_binary_matrix(a);
    validate_binary_matrix(b);

    let n = a.len();
    let m = if n == 0 { 0 } else { a[0].len() };
    let bm = b.len();
    let k = if bm == 0 { 0 } else { b[0].len() };

    assert!(
        m == bm,
        "Dimension mismatch: A is n×m, B is bm×k, need m == bm."
    );

    if n == 0 || k == 0 {
        return vec![String::new(); n];
    }

    let m_packed = packed_word_len(m);
    let a_packed = pack_binary_rows(a);
    let b_packed = pack_binary_cols(b);

    let mut out = vec![String::with_capacity(k); n];
    for i in 0..n {
        let mut row = String::with_capacity(k);
        for j in 0..k {
            let mut parity = 0u32;
            for w in 0..m_packed {
                parity ^= (a_packed[i][w] & b_packed[j][w]).count_ones() & 1;
            }
            row.push(bit_to_char(parity as u8));
        }
        out[i] = row;
    }

    out
}

/// 正方行列 A の det(A) mod 2 を返す（0 or 1）。
/// 入力は 0/1 文字列配列。
pub fn determinant_mod2_from_strings(rows: &[String]) -> u8 {
    let n = rows.len();
    if n == 0 {
        return 1;
    }
    validate_binary_matrix(rows);
    assert!(
        rows[0].len() == n,
        "Determinant requires a square matrix (n×n)."
    );

    let w = packed_word_len(n);
    let mut a = pack_binary_rows(rows);

    // GF(2) ガウス消去
    for col in 0..n {
        let mut pivot = col;
        while pivot < n && ((a[pivot][col / 64] >> (col % 64)) & 1) == 0 {
            pivot += 1;
        }
        if pivot == n {
            return 0;
        }

        if pivot != col {
            a.swap(pivot, col);
            // mod 2 では -1 ≡ 1 のため、swap の符号反転は無視可能
        }

        for row in (col + 1)..n {
            if ((a[row][col / 64] >> (col % 64)) & 1) == 1 {
                for block in (col / 64)..w {
                    a[row][block] ^= a[col][block];
                }
            }
        }
    }

    1
}

/// 正方行列 A の逆行列 A^{-1} を GF(2) 上で求める。
/// 入力は 0/1 文字列配列。
///
/// - 可逆な場合: `Some(inv)`（`inv` は 0/1 文字列配列）
/// - 特異な場合: `None`
pub fn inverse_mod2_from_strings(rows: &[String]) -> Option<Vec<String>> {
    let n = rows.len();
    if n == 0 {
        return Some(vec![]);
    }

    validate_binary_matrix(rows);
    assert!(
        rows[0].len() == n,
        "Inverse requires a square matrix (n×n)."
    );

    let w = packed_word_len(n);
    let mut a = pack_binary_rows(rows);
    let mut inv = vec![vec![0u64; w]; n];

    for i in 0..n {
        inv[i][i / 64] |= 1u64 << (i % 64);
    }

    for col in 0..n {
        let mut pivot = col;
        while pivot < n && ((a[pivot][col / 64] >> (col % 64)) & 1) == 0 {
            pivot += 1;
        }
        if pivot == n {
            return None;
        }

        if pivot != col {
            a.swap(pivot, col);
            inv.swap(pivot, col);
        }

        for row in 0..n {
            if row == col {
                continue;
            }
            if ((a[row][col / 64] >> (col % 64)) & 1) == 1 {
                for block in (col / 64)..w {
                    a[row][block] ^= a[col][block];
                }
                for block in 0..w {
                    inv[row][block] ^= inv[col][block];
                }
            }
        }
    }

    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut bytes = vec![0u8; n];
        for j in 0..n {
            bytes[j] = ((inv[i][j / 64] >> (j % 64)) & 1) as u8;
        }
        out.push(bytes_to_string(&bytes));
    }

    Some(out)
}

/// 行列 A の rank(A) mod 2（GF(2) 上の階数）を返す。
/// 入力は 0/1 文字列配列（長方形可）。
pub fn rank_mod2_from_strings(rows: &[String]) -> usize {
    let n = rows.len();
    if n == 0 {
        return 0;
    }
    validate_binary_matrix(rows);
    let m = rows[0].len();
    if m == 0 {
        return 0;
    }

    let w = packed_word_len(m);
    let mut a = pack_binary_rows(rows);

    // GF(2) ガウス消去でピボット本数を数える
    let mut rank = 0usize;
    for col in 0..m {
        let mut pivot = rank;
        while pivot < n && ((a[pivot][col / 64] >> (col % 64)) & 1) == 0 {
            pivot += 1;
        }
        if pivot == n {
            continue;
        }

        if pivot != rank {
            a.swap(pivot, rank);
        }

        for row in (rank + 1)..n {
            if ((a[row][col / 64] >> (col % 64)) & 1) == 1 {
                for block in (col / 64)..w {
                    a[row][block] ^= a[rank][block];
                }
            }
        }

        rank += 1;
        if rank == n {
            break;
        }
    }

    rank
}

/// 連立一次方程式 A x = b を GF(2) で解く。
///
/// - `rows`: A の各行を 0/1 文字列で与える（n×m）
/// - `rhs`: b を長さ n の 0/1 文字列で与える
///
/// 解が存在しない場合は `None`、存在する場合は
///
/// - 1 つの特解（長さ m の 0/1 文字列）
/// - 同次系 A x = 0 の基底ベクトル列（各要素は長さ m の 0/1 文字列）
///
/// を返す。
///
/// したがって一般解は `x = particular ^ (basis の線形結合)` で表せる。
pub fn solve_linear_system_mod2_from_strings(
    rows: &[String],
    rhs: &str,
) -> Option<(String, Vec<String>)> {
    let n = rows.len();
    validate_binary_matrix(rows);
    assert!(
        rhs.len() == n,
        "rhs length must be equal to number of rows."
    );

    if n == 0 {
        return Some((String::new(), vec![]));
    }

    let m = rows[0].len();
    if m == 0 {
        return if rhs.as_bytes().iter().all(|&c| c == b'0') {
            Some((String::new(), vec![]))
        } else {
            None
        };
    }

    let w = packed_word_len(m);
    let mut a = pack_binary_rows(rows);
    let mut b = pack_binary_vector(rhs);

    let mut rank = 0usize;
    let mut pivot_cols = Vec::new();

    for col in 0..m {
        let mut pivot = rank;
        while pivot < n && ((a[pivot][col / 64] >> (col % 64)) & 1) == 0 {
            pivot += 1;
        }
        if pivot == n {
            continue;
        }

        if pivot != rank {
            a.swap(pivot, rank);
            b.swap(pivot, rank);
        }

        for row in (rank + 1)..n {
            if ((a[row][col / 64] >> (col % 64)) & 1) == 1 {
                for block in (col / 64)..w {
                    a[row][block] ^= a[rank][block];
                }
                b[row] ^= b[rank];
            }
        }

        pivot_cols.push(col);
        rank += 1;
        if rank == n {
            break;
        }
    }

    for row in rank..n {
        let all_zero = a[row].iter().all(|&block| block == 0);
        if all_zero && b[row] == 1 {
            return None;
        }
    }

    // 上側も消去して RREF 化（pivot 列を単位ベクトル化）
    for i in (0..rank).rev() {
        let col = pivot_cols[i];
        for row in 0..i {
            if ((a[row][col / 64] >> (col % 64)) & 1) == 1 {
                for block in (col / 64)..w {
                    a[row][block] ^= a[i][block];
                }
                b[row] ^= b[i];
            }
        }
    }

    // 特解: 自由変数を 0 に固定すると、pivot 変数はそのまま b に一致
    let mut x = vec![0u8; m];
    for i in 0..rank {
        x[pivot_cols[i]] = b[i];
    }

    let mut is_pivot_col = vec![false; m];
    for &col in &pivot_cols {
        is_pivot_col[col] = true;
    }

    let free_cols = (0..m).filter(|&col| !is_pivot_col[col]).collect::<Vec<_>>();

    let mut basis = Vec::with_capacity(free_cols.len());
    for &free_col in &free_cols {
        let mut v = vec![0u8; m];
        v[free_col] = 1;

        // RREF 後は pivot 行の free_col 成分を読むだけで同次解基底が得られる
        for i in 0..rank {
            let col = pivot_cols[i];
            v[col] = ((a[i][free_col / 64] >> (free_col % 64)) & 1) as u8;
        }

        basis.push(bytes_to_string(&v));
    }

    Some((bytes_to_string(&x), basis))
}
