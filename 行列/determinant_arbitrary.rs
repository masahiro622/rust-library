/// 行列式 (任意のmod m)
/// 計算量: O(N^3 log m)
pub fn determinant_arbitrary_mod(mut mat: Vec<Vec<i64>>, m: i64) -> i64 {
    let n = mat.len();
    assert!(
        mat.iter().all(|row| row.len() == n),
        "Matrix must be square"
    );

    let mut det = 1;

    for i in 0..n {
        for j in (i + 1)..n {
            // 行i と 行j の間でユークリッドの互除法を行い、mat[j][i] を 0 にする
            // mat[j][i] が 0 になるまで、「割って引いて交換」を繰り返す
            while mat[j][i] != 0 {
                let ratio = mat[i][i] / mat[j][i];

                // 行操作: mat[i] -= ratio * mat[j]
                for k in i..n {
                    let sub = (ratio as i128 * mat[j][k] as i128 % m as i128) as i64;
                    mat[i][k] = (mat[i][k] - sub) % m;
                    if mat[i][k] < 0 {
                        mat[i][k] += m;
                    }
                }

                // 行を交換 (行列式の符号を反転)
                // 次のループでは、小さくなった mat[i][i] (元のmat[j][i]の余り) が分母側に来る
                mat.swap(i, j);
                det = (m - det) % m;
            }
        }

        // 対角成分が0になった場合、行列式は0
        if mat[i][i] == 0 {
            return 0;
        }

        // 対角成分を累積
        det = (det as i128 * mat[i][i] as i128 % m as i128) as i64;
    }

    det
}
