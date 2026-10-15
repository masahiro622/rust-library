type Matrix = Vec<Vec<Mint>>;
/// 行列の積を返す (mod m) O(n^3)
fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let k = a[0].len();
    let p = b[0].len();

    assert_eq!(
        k,
        b.len(),
        "Matrix dimensions do not match for multiplication."
    );

    let mut res = vec![vec![Mint::raw(0); p]; n];

    for i in 0..n {
        for j in 0..p {
            let mut sum = Mint::raw(0);
            for t in 0..k {
                sum += a[i][t] * b[t][j];
            }
            res[i][j] = sum;
        }
    }
    res
}

/// 単位行列を生成
pub fn identity(n: usize) -> Matrix {
    let mut id = vec![vec![Mint::raw(0); n]; n];
    for i in 0..n {
        id[i][i] = Mint::raw(1);
    }
    id
}

/// 行列のトレース
fn trace(a: &Matrix) -> Mint {
    let n = a.len();
    let mut res = Mint::raw(0);
    for i in 0..n {
        res += a[i][i];
    }
    res
}

/// スカラー倍
fn scalar_mul(a: &Matrix, x: Mint) -> Matrix {
    let n = a.len();
    let mut res = vec![vec![Mint::raw(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            res[i][j] = a[i][j] * x;
        }
    }
    res
}

/// 行列 a の e 乗を返す (mod m)
pub fn mat_pow(mut a: Matrix, mut e: i64) -> Matrix {
    let n = a.len();
    let mut result = identity(n);
    while e > 0 {
        if e % 2 == 1 {
            result = matmul(&result, &a);
        }
        a = matmul(&a, &a);
        e /= 2;
    }
    result
}

fn mat_add_mod(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let p = a[0].len();
    assert_eq!(b.len(), n);
    assert_eq!(b[0].len(), p);

    let mut res = vec![vec![Mint::raw(0); p]; n];
    for i in 0..n {
        for j in 0..p {
            res[i][j] = a[i][j] + b[i][j];
        }
    }
    res
}

fn mat_sub_mod(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let p = a[0].len();
    assert_eq!(b.len(), n);
    assert_eq!(b[0].len(), p);

    let mut res = vec![vec![Mint::raw(0); p]; n];
    for i in 0..n {
        for j in 0..p {
            res[i][j] = a[i][j] - b[i][j];
        }
    }
    res
}

/// 行列式 (mod m)
fn determinant_mod(mut mat: Matrix) -> Mint {
    let n = mat.len();
    assert!(mat.iter().all(|row| row.len() == n), "Not a square matrix");

    let mut det = Mint::raw(1);
    for i in 0..n {
        // ピボット選択
        let mut pivot = i;
        while pivot < n && mat[pivot][i] == Mint::raw(0) {
            pivot += 1;
        }
        if pivot == n {
            return Mint::raw(0); // ランク落ち → det = 0
        }
        if i != pivot {
            mat.swap(i, pivot);
            det = -det; // 行を交換 → 符号反転
        }

        det *= mat[i][i];
        let inv = mat[i][i].inv();

        for j in (i + 1)..n {
            let f = mat[j][i] * inv;
            for k in i..n {
                mat[j][k] = mat[j][k] - f * mat[i][k];
            }
        }
    }
    det
}

/// 階数 (mod m)
fn rank(mut mat: Matrix) -> usize {
    let n = mat.len();
    if n == 0 {
        return 0;
    }
    let m_col = mat[0].len();
    let mut rank = 0;

    for col in 0..m_col {
        // ピボット探し
        let mut pivot = None;
        for row in rank..n {
            if mat[row][col] != Mint::raw(0) {
                pivot = Some(row);
                break;
            }
        }
        if let Some(pivot_row) = pivot {
            mat.swap(rank, pivot_row);
            // ピボット行を正規化
            let inv = mat[rank][col].inv();
            for j in col..m_col {
                mat[rank][j] = mat[rank][j] * inv;
            }
            // 他の行から消去
            for i in 0..n {
                if i != rank && mat[i][col] != Mint::raw(0) {
                    let factor = mat[i][col];
                    for j in col..m_col {
                        mat[i][j] = mat[i][j] - factor * mat[rank][j];
                    }
                }
            }
            rank += 1;
        }
    }
    rank
}
/// Ax = b の一般解を求める
/// - 解が存在しない場合: `None` を返す
/// - 解が存在する場合: Some((particular, basis)) を返す
///     - particular: 特定解 Vec<Mint>
///     - basis: Vec<Vec<Mint>> - 解空間の基底
fn solve_linear_system(mut a: Matrix, b: Vec<Mint>) -> Option<(Vec<Mint>, Vec<Vec<Mint>>)> {
    let n = a.len();
    if n == 0 {
        return Some((vec![], vec![]));
    }
    let m = a[0].len();

    assert_eq!(b.len(), n);

    // 拡大係数行列
    for i in 0..n {
        a[i].push(b[i]);
    }

    let mut rank = 0;
    let mut where_col = vec![None; m]; // 各列のピボット行（なければNone）

    for col in 0..m {
        let mut pivot = None;
        for row in rank..n {
            if a[row][col] != Mint::raw(0) {
                pivot = Some(row);
                break;
            }
        }
        if let Some(pivot_row) = pivot {
            a.swap(rank, pivot_row);
            let inv = a[rank][col].inv();
            for j in col..=m {
                a[rank][j] = a[rank][j] * inv;
            }

            for i in 0..n {
                if i != rank && a[i][col] != Mint::raw(0) {
                    let factor = a[i][col];
                    for j in col..=m {
                        a[i][j] = a[i][j] - factor * a[rank][j];
                    }
                }
            }
            where_col[col] = Some(rank);
            rank += 1;
        }
    }

    // 一致しない行があれば無解
    for i in rank..n {
        if a[i][m] != Mint::raw(0) {
            return None;
        }
    }

    // 特定解を構成
    let mut particular = vec![Mint::raw(0); m];
    for j in 0..m {
        if let Some(row) = where_col[j] {
            particular[j] = a[row][m];
        }
    }

    // 基底ベクトルを構成
    let mut basis = vec![];
    for j in 0..m {
        if where_col[j].is_none() {
            let mut vec = vec![Mint::raw(0); m];
            vec[j] = Mint::raw(1);
            for k in 0..m {
                if let Some(row) = where_col[k] {
                    vec[k] = -a[row][j];
                }
            }
            basis.push(vec);
        }
    }

    Some((particular, basis))
}

/// Z/998244353Z 上で正方行列の逆行列を返す。存在しない場合は None。
fn inverse_matrix(mut a: Matrix) -> Option<Matrix> {
    let n = a.len();
    if a.iter().any(|row| row.len() != n) {
        return None; // 非正方行列
    }

    let mut inv = identity(n);

    for i in 0..n {
        // ピボット探し
        let mut pivot = i;
        while pivot < n && a[pivot][i] == Mint::raw(0) {
            pivot += 1;
        }
        if pivot == n {
            return None; // 逆行列なし
        }

        // 行を交換
        a.swap(i, pivot);
        inv.swap(i, pivot);

        // ピボットの正規化
        let inv_pivot = a[i][i].inv();
        for j in 0..n {
            a[i][j] = a[i][j] * inv_pivot;
            inv[i][j] = inv[i][j] * inv_pivot;
        }

        // 他の行を消去
        for k in 0..n {
            if k != i && a[k][i] != Mint::raw(0) {
                let factor = a[k][i];
                for j in 0..n {
                    a[k][j] = a[k][j] - factor * a[i][j];
                    inv[k][j] = inv[k][j] - factor * inv[i][j];
                }
            }
        }
    }

    Some(inv)
}

/// Z/998244353Z 上で正方行列の随伴行列（adjugate）を返す。
/// 定義に従い、余因子行列の転置を構成する。
pub fn adjugate_matrix(a: &Matrix) -> Matrix {
    let n = a.len();
    assert!(a.iter().all(|row| row.len() == n), "Not a square matrix");

    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![vec![Mint::raw(1)]];
    }

    let r = rank(a.clone());
    if r <= n - 2 {
        return vec![vec![Mint::raw(0); n]; n];
    }

    if r == n {
        let det = determinant_mod(a.clone());
        if let Some(inv) = inverse_matrix(a.clone()) {
            return scalar_mul(&inv, det);
        }
        return vec![vec![Mint::raw(0); n]; n];
    }

    let zero_b = vec![Mint::raw(0); n];
    let x_basis = solve_linear_system(a.clone(), zero_b.clone())
        .expect("homogeneous system must be solvable")
        .1;

    let mut at = vec![vec![Mint::raw(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            at[j][i] = a[i][j];
        }
    }
    let y_basis = solve_linear_system(at, zero_b)
        .expect("homogeneous transposed system must be solvable")
        .1;

    assert!(
        !x_basis.is_empty() && !y_basis.is_empty(),
        "rank n-1 requires non-trivial null vectors"
    );
    let x = &x_basis[0];
    let y = &y_basis[0];

    let j0 = x
        .iter()
        .position(|&v| v != Mint::raw(0))
        .expect("null vector x is unexpectedly zero");
    let i0 = y
        .iter()
        .position(|&v| v != Mint::raw(0))
        .expect("null vector y is unexpectedly zero");

    let mut minor = Vec::with_capacity(n - 1);
    for r0 in 0..n {
        if r0 == i0 {
            continue;
        }
        let mut row = Vec::with_capacity(n - 1);
        for c0 in 0..n {
            if c0 == j0 {
                continue;
            }
            row.push(a[r0][c0]);
        }
        minor.push(row);
    }

    let mut cofactor = determinant_mod(minor);
    if (i0 + j0) % 2 == 1 {
        cofactor = -cofactor;
    }

    let denom = x[j0] * y[i0];
    let alpha = cofactor * denom.inv();

    let mut adj = vec![vec![Mint::raw(0); n]; n];
    for i in 0..n {
        let yi = y[i];
        for j in 0..n {
            let xj = x[j];
            adj[j][i] = alpha * xj * yi;
        }
    }

    adj
}
